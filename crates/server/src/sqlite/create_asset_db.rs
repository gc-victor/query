use tracing::error;

use super::connect_db::connect_asset_db;

pub(crate) fn create_asset_db() {
    match connect_asset_db() {
        Ok(connection) => {
            match connection
                .execute_batch(&["BEGIN;".to_string(), asset(), "COMMIT;".to_string()].join("\n"))
            {
                Ok(_) => (),
                Err(error) => error!("Can't create asset database: {}", error),
            }
        }
        Err(error) => error!("Can't connect to the asset database: {}", error),
    }
}

fn asset() -> String {
    r#"
        CREATE TABLE IF NOT EXISTS asset(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            active BOOLEAN NOT NULL,
            data BLOB NOT NULL,
            name TEXT NOT NULL UNIQUE,
            name_hashed TEXT NOT NULL UNIQUE,
            mime_type TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        CREATE INDEX IF NOT EXISTS asset_idx_name ON asset(name);
        CREATE INDEX IF NOT EXISTS asset_idx_active_name_name_hashed ON asset(active, name, name_hashed);

        CREATE TRIGGER IF NOT EXISTS trigger_asset_update
            AFTER UPDATE ON asset
        BEGIN
            UPDATE
                asset
            SET
                updated_at=(strftime('%s', 'now'))
            WHERE
                id=OLD.id;
        END;
    "#
    .to_string()
}
