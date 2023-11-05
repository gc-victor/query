use anyhow::Result;
use rusqlite::named_params;
use tracing::instrument;

use crate::sqlite::connect_db::connect_config_db;

use super::{
    get_claims::get_claims,
    http_error::{internal_server_error, unauthorized, HttpError},
};

#[instrument(err(Debug), skip(token))]
pub fn validate_is_admin(token: &str) -> Result<(), HttpError> {
    match is_admin(token) {
        Ok(true) => Ok(()),
        Ok(false) => Err(unauthorized()),
        Err(e) => {
            tracing::error!("{e}");
            Err(internal_server_error(e.to_string()))
        }
    }
}

#[instrument(err(Debug), skip(token))]
pub fn is_admin(token: &str) -> Result<bool> {
    let connect = connect_config_db()?;

    let claims = get_claims(token)?;
    let exp = claims.exp;
    let iat = claims.iat;

    if claims.iss != "user_token" {
        return Ok(false);
    };

    match connect.query_row(
        "
        SELECT
            COUNT(*)
        FROM
            _config_user_token t
        LEFT JOIN
            _config_user u
        ON
            u.uuid = t.user_uuid
        WHERE
            t.token = :token
        AND
            t.expiration_date = :exp
        AND
            t.updated_at = :iat
        AND 
            (t.expiration_date > strftime('%s', datetime('now')) OR t.expiration_date = t.updated_at)
        AND
            t.active = 1
        AND
            u.active = 1
        AND
            u.admin = 1;
        ",
        named_params! {
            ":token": token,
            ":exp": exp,
            ":iat": iat,
        },
        |row| row.get(0),
    ) {
        Ok(Some(0)) => {
            tracing::error!("0");
            Ok(false)
        }
        Ok(Some(_)) => Ok(true),
        Ok(None) => {
            tracing::error!("None");
            Ok(false)
        }
        Err(e) => {
            tracing::error!("{e}");
            Ok(false)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::db_test;

    use super::*;

    db_test!(test_validate_is_admin_token, TestValidateIsAdminToken, {
        let conn = connect_config_db().unwrap();

        conn.execute(
            "
            INSERT INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'is-admin',
                'password',
                1
            );
            ",
            (),
        )
        .unwrap();

        let token: String = conn
        .query_row(
            "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'is-admin')",
            [],
            |row| row.get(0),
        )
        .unwrap();

        assert!(is_admin(&token).unwrap());
        assert!(validate_is_admin(&token).is_ok());
    });

    db_test!(
        test_validate_is_admin_non_admin_token,
        TestValidateIsAdminNonAdminToken,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
                "
            INSERT INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'non-admin',
                'password',
                0
            );
            ",
                (),
            )
            .unwrap();

            let token: String = conn
        .query_row(
            "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'non-admin')",
            [],
            |row| row.get(0),
        )
        .unwrap();

            assert!(!is_admin(&token).unwrap());
            assert_eq!(validate_is_admin(&token).unwrap_err(), unauthorized());
        }
    );

    db_test!(
        test_validate_is_admin_invalid_token,
        TestValidateIsAdminInvalidToken,
        {
            let conn = connect_config_db().unwrap();

            conn.execute(
                "
            INSERT INTO _config_user(
                email,
                password,
                admin
            ) VALUES (
                'invalid-token',
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
                user_uuid = (SELECT uuid FROM _config_user WHERE email = 'invalid-token');
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
                    (SELECT uuid FROM _config_user WHERE email = 'invalid-token'),
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "invalid_token"}'),
                    strftime('%s', datetime('now')),
                    1
                );
            "#,
            (),
        )
        .unwrap();

            let token: String = conn
        .query_row(
            "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'invalid-token')",
            [],
            |row| row.get(0),
        )
        .unwrap();

            assert!(!is_admin(&token).unwrap());
            assert_eq!(validate_is_admin(&token).unwrap_err(), unauthorized());
        }
    );
}
