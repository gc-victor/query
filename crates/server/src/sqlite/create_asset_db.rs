use super::connect_db::connect_asset_db;

pub(crate) fn create_asset_db() {
    // TODO: remove expects and add a proper error handling
    connect_asset_db()
        .expect("Can't connect to the asset database")
        .execute_batch(&["BEGIN;".to_string(), asset(), "COMMIT;".to_string()].join("\n"))
        .expect("Can't create asset database");
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

        CREATE INDEX IF NOT EXISTS asset_idx_active ON asset(active);

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
