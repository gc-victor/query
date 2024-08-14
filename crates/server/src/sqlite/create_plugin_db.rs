use tracing::error;

use super::connect_db::connect_plugin_db;

pub fn create_plugin_db() {
    match connect_plugin_db() {
        Ok(connection) => {
            match connection.execute_batch(
                &[
                    "BEGIN;".to_string(),
                    create_plugin_table(),
                    "COMMIT;".to_string(),
                ]
                .join("\n"),
            ) {
                Ok(_) => (),
                Err(err) => error!("Can't create plugin database: {}", err),
            }
        }
        Err(err) => error!("Can't connect to the plugin database: {}", err),
    }
}

fn create_plugin_table() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS plugin(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            data BLOB NOT NULL,
            name TEXT NOT NULL UNIQUE,
            sha256 TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        CREATE INDEX IF NOT EXISTS plugin_idx_name_plugin ON plugin(name);

        CREATE TRIGGER IF NOT EXISTS trigger_plugin_update
            AFTER UPDATE ON plugin
        BEGIN
            UPDATE
                plugin
            SET
                updated_at = (strftime('%s', datetime('now')))
            WHERE
                id = OLD.id;
        END;
    "#
    .to_string()
}
