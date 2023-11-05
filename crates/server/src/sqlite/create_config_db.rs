use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::env::Env;

use super::connect_db::connect_config_db;

pub fn create_config_db() {
    connect_config_db()
        .expect("Can't connect to the config database")
        .execute_batch(
            &[
                "BEGIN;".to_string(),
                create_user_table(),
                insert_admin_user(),
                create_user_token_table(),
                insert_admin_user_token(),
                create_token_table(),
                options(),
                "COMMIT;".to_string(),
            ]
            .join("\n"),
        )
        .expect("Can't create config database");
}

fn create_user_table() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS _config_user(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE CHECK (uuid != '') DEFAULT (uuid()),
            email TEXT NOT NULL UNIQUE CHECK (email != ''),
            password TEXT NOT NULL CHECK (password != ''),
            admin BOOLEAN NOT NULL CHECK (admin IN (0, 1)) DEFAULT (0),
            active BOOLEAN NOT NULL CHECK (active IN (0, 1)) DEFAULT (1),
            created_at INTEGER DEFAULT (strftime('%s', datetime('now'))),
            updated_at INTEGER DEFAULT (strftime('%s', datetime('now')))
        );

        CREATE TRIGGER IF NOT EXISTS _trigger_config_user_update
            AFTER UPDATE ON _config_user
        BEGIN
            UPDATE
                _config_user
            SET
                updated_at = (strftime('%s', datetime('now')))
            WHERE
                id = OLD.id;
        END;
    "#
    .to_string()
}

fn insert_admin_user() -> String {
    let email: String = Env::admin_email();
    let password = Env::admin_password();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    format!(
        r#"INSERT OR IGNORE INTO _config_user(
            email,
            password,
            admin
        ) VALUES (
            '{email}',
            '{hash_password}',
            1
        );"#
    )
}

fn create_user_token_table() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS _config_user_token(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_uuid INTEGER NOT NULL UNIQUE,
            token TEXT NOT NULL UNIQUE CHECK (token != ''),
            expiration_date INTEGER NOT NULL DEFAULT (strftime('%s', datetime('now', '+1 month'))),
            active BOOLEAN NOT NULL CHECK (active IN (0, 1)) DEFAULT (1),
            write BOOLEAN NOT NULL CHECK (write IN (0, 1)) DEFAULT (0),
            created_at INTEGER DEFAULT (strftime('%s', datetime('now'))),
            updated_at INTEGER DEFAULT (strftime('%s', datetime('now'))),
            FOREIGN KEY (user_uuid) REFERENCES _config_user(uuid) ON DELETE CASCADE
        );

        CREATE TRIGGER IF NOT EXISTS _trigger_config_user_token_update
            AFTER UPDATE ON _config_user_token
        BEGIN
            UPDATE
                _config_user_token
            SET
                token = token('{
                    "sub": "' || (SELECT uuid()) ||  '",
                    "exp": ' || COALESCE(NEW.expiration_date, OLD.expiration_date) || ',
                    "iat": ' || NEW.updated_at || ',
                    "iss": "user_token"
                }'),
                updated_at = (strftime('%s', datetime('now')))
            WHERE
                id = OLD.id;
        END;

        CREATE TRIGGER IF NOT EXISTS _trigger_config_user_token_on_user_insert
            AFTER INSERT ON _config_user
        BEGIN
            INSERT OR IGNORE INTO
                _config_user_token(
                    user_uuid,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    NEW.uuid,
                    token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}'),
                    strftime('%s', datetime('now')),
                    1
                );
        END;
    "#
    .to_string()
}

// TODO: create a table adding restrictions by database name, action (migration and query) and operations (SELECT, INSERT, DELETE, UPDATE) for each user token

fn insert_admin_user_token() -> String {
    let email: String = Env::admin_email();

    format!(
        r#"
            INSERT OR IGNORE INTO
                _config_user_token(
                    user_uuid,
                    token,
                    expiration_date,
                    write
                )
            VALUES
                (
                    (SELECT uuid FROM _config_user WHERE email = '{email}'),
                    token('{{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}}'),
                    strftime('%s', datetime('now')),
                    1
                );
            "#,
    )
}

fn create_token_table() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS _config_token(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE CHECK (name != ''),
            token TEXT NOT NULL UNIQUE CHECK (token != ''),
            expiration_date INTEGER NOT NULL DEFAULT (strftime('%s', datetime('now', '+1 month'))),
            active BOOLEAN NOT NULL CHECK (active IN (0, 1)) DEFAULT (1),
            write BOOLEAN NOT NULL CHECK (write IN (0, 1)) DEFAULT (0),
            created_at INTEGER DEFAULT (strftime('%s', datetime('now'))),
            updated_at INTEGER DEFAULT (strftime('%s', datetime('now')))
        );

        CREATE TRIGGER IF NOT EXISTS _trigger_config_token_update
            AFTER UPDATE ON _config_token
        BEGIN
            UPDATE
                _config_token
            SET
                token = token('{
                    "sub": "' || (SELECT uuid()) ||  '",
                    "exp": ' || COALESCE(NEW.expiration_date, OLD.expiration_date) || ',
                    "iat": ' || NEW.updated_at || ',
                    "iss": "token"
                }'),
                updated_at = (strftime('%s', datetime('now')))
            WHERE
                id = OLD.id;
        END;
    "#
    .to_string()
}

// TODO: create a table adding restrictions by database name, action (migration and query) and operations (SELECT, INSERT, DELETE, UPDATE) for each token

// IMPORTANT! Insert the initial name and value for each option
fn options() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS _config_option(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL DEFAULT ('0')
        );

        CREATE TRIGGER IF NOT EXISTS _trigger_config_option_delete
            BEFORE DELETE ON _config_option
        BEGIN
            SELECT RAISE(FAIL, 'It is not allowed to delete a row from _config_option');
        END;

        -- IMPORTANT! Insert the initial name and value state for each option
        -- Allows the creation of users
        INSERT OR IGNORE INTO _config_option (name, value) VALUES ('create_user', '1');
        -- Allows the creation of tokens
        INSERT OR IGNORE INTO _config_option (name, value) VALUES ('create_token', '1');
    "#
    .to_string()
}
