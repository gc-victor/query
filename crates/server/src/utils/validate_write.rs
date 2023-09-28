use rusqlite::{named_params, Connection};
use tracing::instrument;

use crate::{
    sqlite::connect_db::connect_config_db,
    utils::{
        get_claims::get_claims,
        http_error::{internal_server_error, unauthorized},
    },
    HttpError,
};

#[instrument(err(Debug), skip(token))]
pub fn validate_write(token: &str) -> Result<bool, HttpError> {
    let connect: Result<Connection, HttpError> = match connect_config_db() {
        Ok(c) => Ok(c),
        Err(e) => return Err(internal_server_error(e.to_string())),
    };

    let claims = get_claims(token)?;
    let exp = claims.exp;
    let iat = claims.iat;

    let table = match claims.iss.as_str() {
        "user_token" => "_config_user_token",
        "token" => "_config_token",
        _ => return Err(unauthorized()),
    };

    match connect?.query_row(
        &format!(
            "
            SELECT
                COUNT(*)
            FROM
                {table}
            WHERE
                token = :token
            AND
                expiration_date = :exp
            AND
                updated_at = :iat
            AND 
                (expiration_date > strftime('%s', datetime('now')) OR expiration_date = updated_at)
            AND
                active = 1
            AND
                write = 1;
        "
        ),
        named_params! {
            ":token": token,
            ":exp": exp,
            ":iat": iat,
        },
        |row| row.get(0),
    ) {
        Ok(Some(0)) => Ok(false),
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => {
            tracing::error!("{}", e.to_string());
            Err(internal_server_error(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{encode, EncodingKey, Header};

    use crate::utils::get_claims::Claims;

    use super::*;

    use crate::db_test;

    db_test!(
        test_validate_write_invalid_iss,
        TestValidateWriteInvalidIss,
        {
            let claims = Claims {
                iss: "invalid_issuer".to_owned(),
                exp: 1_000_000_000,
                iat: 1_000_000_000,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .unwrap();

            let result = validate_write(&token);

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), unauthorized());
        }
    );

    db_test!(
        test_validate_write_token_with_expire_equals_update,
        TestValidateWriteTokenWithExpireEqualsUpdate,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
            r#"
            INSERT INTO
                _config_token(
                    name,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    'test',
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "token"}'),
                    strftime('%s', datetime('now')),
                    1
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
                .query_row("SELECT token FROM _config_token WHERE id = 1", [], |row| {
                    row.get(0)
                })
                .unwrap();

            let result = validate_write(&token);

            assert!(result.unwrap());
        }
    );

    db_test!(
        test_validate_write_token_false,
        TestValidateWriteTokenFalse,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
            r#"
            INSERT INTO
                _config_token(
                    name,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    'test',
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "token"}'),
                    strftime('%s', datetime('now')),
                    0
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
                .query_row("SELECT token FROM _config_token WHERE id = 1", [], |row| {
                    row.get(0)
                })
                .unwrap();

            let result = validate_write(&token);

            assert!(result.is_ok());
            assert!(!result.unwrap());
        }
    );

    db_test!(
        test_validate_write_token_with_greater_expire_date,
        TestValidateWriteTokenWithGreaterExpireDate,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
            r#"
            INSERT OR IGNORE INTO
                _config_token(
                    name,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    'test',
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now'), '+1 month') || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "token"}'),
                    strftime('%s', datetime('now', '+1 month')),
                    1
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
                .query_row("SELECT token FROM _config_token WHERE id = 1", [], |row| {
                    row.get(0)
                })
                .unwrap();

            let result = validate_write(&token);

            assert!(result.is_ok());
        }
    );

    db_test!(
        test_validate_write_expired_token,
        TestValidateWriteExpiredToken,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
            r#"
            INSERT OR IGNORE INTO
                _config_token(name, token, expiration_date, write)
            VALUES
                ('test', token('{"sub": "uuid", "exp": 0, "iat": 0, "iss": "test"}'), strftime('%s', datetime('now', '-1 month')), 1);
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
                .query_row("SELECT token FROM _config_token WHERE id = 1", [], |row| {
                    row.get(0)
                })
                .unwrap();

            let result = validate_write(&token);

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), unauthorized());
        }
    );

    db_test!(
        test_validate_write_token_with_invalid_claim,
        TestValidateWriteTokenWithInvalidClaim,
        {
            let result = validate_write("invalid_token");

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), unauthorized());
        }
    );

    db_test!(
        test_validate_write_user_token_false,
        TestValidateWriteUserTokenFalse,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
                "
            INSERT INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'user-0@test.com',
                'password',
                1
            );
            ",
                (),
            )
            .unwrap();

            conn.execute(
                r#"
            DELETE FROM
                _config_user_token
            WHERE
                user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-0@test.com');
            "#,
                (),
            )
            .unwrap();

            conn.execute(
            r#"
            INSERT INTO
                _config_user_token(
                    user_uuid,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    (SELECT uuid FROM _config_user WHERE email = 'user-0@test.com'),
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}'),
                    strftime('%s', datetime('now')),
                    0
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
            .query_row(
                "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-0@test.com')",
                [],
                |row| row.get(0),
            )
            .unwrap();

            let result = validate_write(&token);

            assert!(result.is_ok());
            assert!(!result.unwrap());
        }
    );

    db_test!(
        test_validate_write_user_token_with_expire_equals_update,
        TestValidateWriteUserTokenWithExpireEqualsUpdate,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
                "
            INSERT INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'user-0@test.com',
                'password',
                1
            );
            ",
                (),
            )
            .unwrap();

            conn.execute(
                r#"
            DELETE FROM
                _config_user_token
            WHERE
                user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-0@test.com');
            "#,
                (),
            )
            .unwrap();

            conn.execute(
            r#"
            INSERT INTO
                _config_user_token(
                    user_uuid,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    (SELECT uuid FROM _config_user WHERE email = 'user-0@test.com'),
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}'),
                    strftime('%s', datetime('now')),
                    1
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
            .query_row(
                "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-0@test.com')",
                [],
                |row| row.get(0),
            )
            .unwrap();

            let result = validate_write(&token);

            assert!(result.is_ok());
        }
    );

    db_test!(
        test_validate_write_user_token_with_greater_expire_date,
        TestValidateWriteUserTokenWithGreaterExpireDate,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
                "
            INSERT INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'user-1@test.com',
                'password',
                1
            );
            ",
                (),
            )
            .unwrap();

            conn.execute(
                r#"
            DELETE FROM
                _config_user_token
            WHERE
                user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-1@test.com');
            "#,
                (),
            )
            .unwrap();

            conn.execute(
            r#"
            INSERT INTO
                _config_user_token(
                    user_uuid,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    (SELECT uuid FROM _config_user WHERE email = 'user-1@test.com'),
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now', '+1 month')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}'),
                    strftime('%s', datetime('now', '+1 month')),
                    1
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
            .query_row(
                "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-1@test.com')",
                [],
                |row| row.get(0),
            )
            .unwrap();

            let result = validate_write(&token);

            assert!(result.is_ok());
        }
    );

    db_test!(
        test_validate_write_expired_user_token,
        TestValidateWriteExpiredUserToken,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
                "
            INSERT OR IGNORE INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'user-2@test.com',
                'password',
                1
            );
            ",
                (),
            )
            .unwrap();

            conn.execute(
                r#"
            DELETE FROM
                _config_user_token
            WHERE
                user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-2@test.com');
            "#,
                (),
            )
            .unwrap();

            conn.execute(
            r#"
            INSERT INTO
                _config_user_token(
                    user_uuid,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    (SELECT uuid FROM _config_user WHERE email = 'user-2@test.com'),
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now', '-1 month')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}'),
                    strftime('%s', datetime('now', '-1 month')),
                    1
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
            .query_row(
                "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user-2@test.com')",
                [],
                |row| row.get(0),
            )
            .unwrap();

            let result = validate_write(&token);

            assert!(result.is_ok());
            assert!(!result.unwrap());
        }
    );

    db_test!(
        test_validate_write_user_token_with_invalid_claim,
        TestValidateWriteUserTokenWithInvalidClaim,
        {
            let result = validate_write("invalid_token");

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), unauthorized());
        }
    );
}
