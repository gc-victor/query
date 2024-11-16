use tracing::error;

use super::connect_db::connect_function_db;

pub fn create_function_db() {
    match connect_function_db() {
        Ok(connection) => {
            match connection.execute_batch(
                &[
                    "BEGIN;".to_string(),
                    create_function_table(),
                    "COMMIT;".to_string(),
                ]
                .join("\n"),
            ) {
                Ok(_) => (),
                Err(err) => error!("Can't create function database: {}", err),
            }
        }
        Err(err) => error!("Can't connect to the function database: {}", err),
    }
}

fn create_function_table() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS function(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            active BOOLEAN NOT NULL CHECK (active IN (0, 1)) DEFAULT 1,
            method TEXT NOT NULL CHECK (method REGEXP '^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH)$'),
            path TEXT NOT NULL,
            function BLOB NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            UNIQUE(method, path)
        );

        CREATE INDEX IF NOT EXISTS function_idx_active_path_method ON function(method, active, path);

        CREATE TRIGGER IF NOT EXISTS trigger_function_update
            AFTER UPDATE ON function
        BEGIN
            UPDATE
                function
            SET
                updated_at = (strftime('%s', datetime('now')))
            WHERE
                id = OLD.id;
        END;
    "#
    .to_string()
}
