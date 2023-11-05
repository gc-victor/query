use crate::sqlite::connect_db::connect_config_db;

use super::http_error::{internal_server_error, not_found, HttpError};

pub fn validate_user_creation() -> Result<(), HttpError> {
    // NOTE: configure to allow or not to create users and projects
    match connect_config_db()?.query_row(
        "SELECT value FROM _config_option WHERE name = 'create_user'",
        [],
        |row| -> std::result::Result<String, rusqlite::Error> { row.get(0) },
    ) {
        Ok(s) if s == "1" => Ok(()),
        Ok(s) if s == "0" => Err(not_found()),
        _ => Err(internal_server_error(
            "Error getting the value of the option 'create_token'".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::db_test;

    use super::*;

    db_test!(
        test_validate_user_creation_allowed,
        TestValidateUserCreationAllowed,
        {
            let result = validate_user_creation();
            assert!(result.is_ok());
        }
    );

    db_test!(
        test_validate_user_creation_not_allowed,
        TestValidateUserCreationNotAllowed,
        {
            let conn = connect_config_db().unwrap();
            conn.execute(
                "UPDATE _config_option SET value = '0' WHERE name = 'create_user'",
                [],
            )
            .unwrap();
            let result = validate_user_creation();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), not_found());
        }
    );
}
