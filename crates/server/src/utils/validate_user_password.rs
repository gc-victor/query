use argon2::{Argon2, PasswordHash, PasswordVerifier};
use tracing::instrument;

use crate::{sqlite::connect_db::connect_config_db, HttpError};

use super::http_error::bad_request;

#[instrument(err(Debug), skip(password))]
pub fn validate_user_password(email: &str, password: &str) -> Result<(), HttpError> {
    let password_hash: String = connect_config_db()
        .unwrap()
        .query_row(
            "SELECT password FROM _config_user WHERE email = ? AND active = 1",
            [email],
            |row| row.get(0),
        )
        .map_err(|_| bad_request("The email or password is not correct.".to_string()))?;

    let is_password_valid = Argon2::default()
        .verify_password(
            password.as_bytes(),
            &PasswordHash::new(&password_hash).unwrap(),
        )
        .is_ok();

    if is_password_valid {
        Ok(())
    } else {
        Err(bad_request(
            "The email or password is not correct.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    use crate::db_test;

    use super::*;

    db_test!(
        test_validate_user_password_valid,
        TestValidateUserPasswordValid,
        {
            let email = "test@example.com";
            let password = "password";
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let hash_password = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            let conn = connect_config_db().unwrap();
            conn.execute(
                "INSERT INTO _config_user (email, password, active) VALUES (?, ?, 1)",
                [email, &hash_password],
            )
            .unwrap();
            let result = validate_user_password(email, password);

            assert!(result.is_ok());
        }
    );

    db_test!(
        test_validate_user_password_invalid_email,
        TestValidateUserPasswordInvalidEmail,
        {
            let email = "test@example.com";
            let password = "password";
            let result = validate_user_password(email, password);

            assert!(result.is_err());
            assert_eq!(
            "HttpError { code: 400 Bad Request, message: The email or password is not correct. }",
            format!("{:?}", result.unwrap_err())
        );
        }
    );

    db_test!(
        test_validate_user_password_invalid_password,
        TestValidateUserPasswordInvalidPassword,
        {
            let email = "test@example.com";
            let password = "password";
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let hash_password = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();
            let conn = connect_config_db().unwrap();
            conn.execute(
                "INSERT INTO _config_user (email, password, active) VALUES (?, ?, 1)",
                [email, &hash_password],
            )
            .unwrap();
            let result = validate_user_password(email, "invalid_password");

            assert!(result.is_err());
            assert_eq!(
            "HttpError { code: 400 Bad Request, message: The email or password is not correct. }",
            format!("{:?}", result.unwrap_err())
        );
        }
    );

    db_test!(
        test_validate_user_password_inactive_user,
        TestValidateUserPasswordInactiveUser,
        {
            let email = "test@example.com";
            let password = "password";
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let hash_password = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();
            let conn = connect_config_db().unwrap();
            conn.execute(
                "INSERT INTO _config_user (email, password, active) VALUES (?, ?, 0)",
                [email, &hash_password],
            )
            .unwrap();
            let result = validate_user_password(email, password);

            assert!(result.is_err());
            assert_eq!(
            "HttpError { code: 400 Bad Request, message: The email or password is not correct. }",
            format!("{:?}", result.unwrap_err())
        );
        }
    );
}
