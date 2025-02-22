use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey, Header};
use rbase64;
use regex::Regex;
use rusqlite::ffi;
use rusqlite::{functions::FunctionFlags, Connection, Error, Result};
use uuid::Uuid;

use crate::env::Env;

pub fn _uuid_function(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "uuid",
        0,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |_ctx| Ok(Uuid::new_v4().to_string()),
    )?;

    Ok(())
}

pub fn _regexp_function(conn: &Connection) -> Result<()> {
    type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

    conn.create_scalar_function(
        "regexp",
        2,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| -> Result<bool, Error> {
            let regexp: Arc<Regex> = ctx.get_or_create_aux(0, |vr| -> Result<_, BoxError> {
                Ok(Regex::new(vr.as_str()?)?)
            })?;
            let is_match = {
                let text = ctx
                    .get_raw(1)
                    .as_str()
                    .map_err(|e| Error::UserFunctionError(e.into()))?;

                regexp.is_match(text)
            };

            Ok(is_match)
        },
    )
}

pub fn _valid_json_function(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "valid_json",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| -> Result<bool, Error> {
            let is_valid = {
                let is: Result<serde_json::Value, serde_json::Error> =
                    serde_json::from_str(ctx.get_raw(0).as_str()?);

                !matches!(is, Err(_err))
            };

            Ok(is_valid)
        },
    )?;

    Ok(())
}

pub fn _token_function(conn: &Connection) -> Result<(), Error> {
    conn.create_scalar_function(
        "token",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| -> Result<String, Error> {
            let claims = ctx.get_raw(0).as_str()?;
            let claims: serde_json::Value = serde_json::from_str(claims).unwrap();

            let token = match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(Env::token_secret().as_ref()),
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(Error::SqlInputError {
                        error: ffi::Error::new(0),
                        msg: e.to_string(),
                        sql: String::new(),
                        offset: -1,
                    })
                }
            };

            Ok(token)
        },
    )
}

pub fn _base64_encode_function(conn: &Connection) -> Result<(), Error> {
    conn.create_scalar_function(
        "base64_encode",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| -> Result<String, Error> {
            let data = ctx.get_raw(0).as_str()?;
            let encoded = rbase64::encode(data.as_bytes());

            Ok(encoded)
        },
    )
}

pub fn _base64_decode_function(conn: &Connection) -> Result<(), Error> {
    conn.create_scalar_function(
        "base64_decode",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| -> Result<String, Error> {
            let data = ctx.get_raw(0).as_str()?;
            let bytes = rbase64::decode(data).expect("Can't decode base64");

            Ok(String::from_utf8(bytes).expect("Can't convert a vector of bytes to String"))
        },
    )
}

pub fn _not_allowed_function(conn: &Connection) -> Result<(), Error> {
    conn.create_scalar_function(
        "not_allowed",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| -> Result<String, Error> {
            Err(Error::SqlInputError {
                error: ffi::Error::new(0),
                msg: ctx.get_raw(0).as_str()?.to_string(),
                sql: String::new(),
                offset: -1,
            })
        },
    )
}

// TODO: add tests
#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use rusqlite::{Connection, Result};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use uuid::{Variant, Version};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub exp: i64,
        pub iat: i64,
        pub iss: String,
    }

    #[test]
    fn test_set_uuid_value() -> Result<(), rusqlite::Error> {
        let conn = Connection::open_in_memory()?;

        _uuid_function(&conn)?;

        let result: String = conn
            .query_row("SELECT uuid()", [], |row| row.get(0))
            .unwrap();

        let parse = Uuid::try_parse(&result).unwrap();

        assert_eq!(Some(Version::Random), parse.get_version());
        assert_eq!(Variant::RFC4122, parse.get_variant());

        Ok(())
    }

    #[test]
    fn test_panic_regex() {
        let conn = Connection::open_in_memory().unwrap();

        _regexp_function(&conn).unwrap();

        conn.execute(
            "CREATE TABLE regexp (
                regexp TEXT NOT NULL CHECK (regexp REGEXP '^(0|1|2)$')
            )",
            (),
        )
        .unwrap();

        let error = conn
            .execute("INSERT INTO regexp (regexp) VALUES ('test')", ())
            .unwrap_err();

        assert_eq!(
            "CHECK constraint failed: regexp REGEXP '^(0|1|2)$'".to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_pass_regex() -> Result<(), rusqlite::Error> {
        let conn = Connection::open_in_memory()?;

        _regexp_function(&conn)?;

        conn.execute(
            "CREATE TABLE regexp (
                regexp TEXT NOT NULL CHECK (regexp REGEXP '^(0|1|2)$')
            )",
            (),
        )?;

        conn.execute("INSERT INTO regexp (regexp) VALUES ('1')", ())?;

        let select: Result<_, rusqlite::Error> =
            conn.query_row("SELECT regexp FROM regexp", [], |row| row.get(0));
        let result = match select {
            Ok(s) => s,
            Err(e) => e.to_string(),
        };

        assert_eq!(result, "1");

        Ok(())
    }

    #[test]
    fn test_panic_valid_json() {
        let conn = Connection::open_in_memory().unwrap();

        _valid_json_function(&conn).unwrap();

        conn.execute(
            "CREATE TABLE json (
                json TEXT NOT NULL CHECK (valid_json(json)=true)
            )",
            (),
        )
        .unwrap();

        let error = conn
            .execute("INSERT INTO json (json) VALUES ('{\"test\": }')", ())
            .unwrap_err();

        assert_eq!(
            "CHECK constraint failed: valid_json(json)=true".to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_pass_valid_json() -> Result<(), rusqlite::Error> {
        let conn = Connection::open_in_memory().unwrap();

        _valid_json_function(&conn)?;

        conn.execute(
            "CREATE TABLE json (
                json TEXT NOT NULL CHECK (valid_json(json)=true)
            )",
            (),
        )?;

        conn.execute("INSERT INTO json (json) VALUES ('{\"test\": 1}')", ())?;

        let select: Result<_, rusqlite::Error> =
            conn.query_row("SELECT json FROM json", [], |row| row.get(0));
        let result = match select {
            Ok(s) => s,
            Err(e) => e.to_string(),
        };

        assert_eq!(result, "{\"test\": 1}");

        Ok(())
    }

    #[test]
    fn test_set_token() -> Result<(), rusqlite::Error> {
        unsafe {
            env::set_var("QUERY_SERVER_TOKEN_SECRET", "secret");
        }

        let conn = Connection::open_in_memory()?;

        _token_function(&conn)?;

        let result: String = conn
            .query_row(
                "SELECT token('{\"sub\": \"uuid\", \"exp\": 10000, \"iat\": 10000, \"iss\": \"test\"}')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        let mut validation = Validation::new(Algorithm::default());
        validation.validate_exp = false;

        let token_data = decode::<Claims>(
            &result,
            &DecodingKey::from_secret(Env::token_secret().as_ref()),
            &validation,
        )
        .unwrap();
        let nw: i64 = 10000;

        assert_eq!(token_data.claims.exp, nw);
        assert_eq!(token_data.claims.iat, nw);
        assert_eq!(token_data.claims.iss, "test");

        Ok(())
    }

    #[test]
    fn test_set_base64() -> Result<(), rusqlite::Error> {
        let conn = Connection::open_in_memory()?;

        _base64_encode_function(&conn)?;

        let result: String = conn
            .query_row("SELECT base64_encode('test')", [], |row| row.get(0))
            .unwrap();

        assert_eq!(result, "dGVzdA==");

        Ok(())
    }

    #[test]
    fn test_get_base64() -> Result<(), rusqlite::Error> {
        let conn = Connection::open_in_memory()?;

        _base64_encode_function(&conn)?;
        _base64_decode_function(&conn)?;

        conn.execute(
            "CREATE TABLE base64_encode (
                base64_encode TEXT
            )",
            (),
        )?;

        conn.execute(
            "INSERT INTO base64_encode (base64_encode) VALUES (base64_encode('test'))",
            (),
        )?;

        let select: Result<_, rusqlite::Error> = conn.query_row(
            "SELECT base64_decode(base64_encode) FROM base64_encode",
            [],
            |row| row.get(0),
        );
        let result = match select {
            Ok(s) => s,
            Err(e) => e.to_string(),
        };

        assert_eq!(result, "test");

        Ok(())
    }

    #[test]
    fn test_not_allow() -> Result<(), rusqlite::Error> {
        let conn = Connection::open_in_memory()?;

        _not_allowed_function(&conn)?;

        conn.execute_batch(
            "
            BEGIN;
            
            CREATE TABLE not_allow (
                not_allow TEXT,
                allow TEXT
            );

            CREATE TRIGGER IF NOT EXISTS _trigger_update
                BEFORE UPDATE OF not_allow ON not_allow
            BEGIN
                SELECT not_allowed('It is not allowed to update table');
            END;

            CREATE TRIGGER IF NOT EXISTS _trigger_delete
                BEFORE DELETE ON not_allow
            BEGIN
                SELECT not_allowed('It is not allowed to delete table');
            END;

            COMMIT;
            ",
        )?;

        conn.execute("INSERT INTO not_allow (not_allow) VALUES ('test')", ())?;

        conn.execute(
            "UPDATE not_allow SET allow = 'test' WHERE not_allow = 'test'",
            (),
        )?;

        assert_eq!(
            "It is not allowed to update table in  at offset -1".to_string(),
            conn.execute("UPDATE not_allow SET not_allow = 'test'", ())
                .unwrap_err()
                .to_string()
        );

        assert_eq!(
            "It is not allowed to delete table in  at offset -1".to_string(),
            conn.execute("DELETE FROM not_allow WHERE not_allow = 'test'", ())
                .unwrap_err()
                .to_string()
        );

        Ok(())
    }
}
