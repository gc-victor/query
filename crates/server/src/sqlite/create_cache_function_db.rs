use tracing::error;

use crate::sqlite::connect_db::connect_cache_function_db;

pub(crate) fn create_cache_function_db() {
    match connect_cache_function_db() {
        Ok(connection) => {
            match connection.execute_batch(
                &[
                    "BEGIN;".to_string(),
                    cache_function(),
                    "COMMIT;".to_string(),
                ]
                .join("\n"),
            ) {
                Ok(_) => (),
                Err(error) => error!("Can't create cache_function database: {}", error),
            }
        }
        Err(error) => error!("Can't connect to the cache_function database: {}", error),
    }
}

fn cache_function() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS cache_function(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            body TEXT NOT NULL,
            headers TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            path TEXT NOT NULL UNIQUE,
            status INTEGER NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        CREATE INDEX IF NOT EXISTS cache_function_idx_active ON cache_function(path);

        CREATE TRIGGER IF NOT EXISTS trigger_asset_update 
            AFTER UPDATE ON cache_function
        BEGIN
            UPDATE
                cache_function
            SET
                updated_at=(strftime('%s', 'now'))
            WHERE
                id=OLD.id;
        END;
    "#
    .to_string()
}
