use rusqlite::Connection;
use tracing::instrument;

use crate::{sqlite::connect_db::connect_config_db, utils::http_error::bad_request};

use super::http_error::{internal_server_error, HttpError};

#[instrument(err(Debug))]
pub fn validate_user_email(email: &str) -> Result<(), HttpError> {
    let connect: Result<Connection, HttpError> = match connect_config_db() {
        Ok(c) => Ok(c),
        Err(e) => return Err(internal_server_error(e.to_string())),
    };

    match connect?.query_row(
        " SELECT COUNT(*) FROM _config_user WHERE email = ?;",
        [email],
        |row| row.get(0),
    ) {
        Ok(Some(0)) => {
            tracing::error!("0");
            Err(bad_request(format!("The email ({email}) don't exist")))
        }
        Ok(Some(_)) => Ok(()),
        Ok(None) => {
            tracing::error!("None");
            Err(bad_request(format!("The email None ({email}) don't exist")))
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            Err(internal_server_error(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db_test;

    use super::*;

    db_test!(
        test_validate_user_email_existing,
        TestValidateUserEmailExisting,
        {
            let email = "test@example.com";
            let conn = connect_config_db().unwrap();
            conn.execute(
                "INSERT INTO _config_user (email, password, active) VALUES (?, ?, 1)",
                [email, "password_hash"],
            )
            .unwrap();
            let result = validate_user_email(email);

            assert!(result.is_ok());
        }
    );

    db_test!(
        test_validate_user_email_non_existing,
        TestValidateUserEmailNonExisting,
        {
            let email = "test@example.com";
            let result = validate_user_email(email);

            assert!(result.is_err());
            assert_eq!(
                format!("{:?}", result.unwrap_err()),
                format!(
                    "HttpError {{ code: 400 Bad Request, message: The email ({}) don't exist }}",
                    email
                )
            );
        }
    );
}
