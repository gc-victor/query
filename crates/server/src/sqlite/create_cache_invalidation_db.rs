use tracing::error;

use crate::sqlite::connect_db::connect_cache_invalidation_db;

pub(crate) fn create_cache_invalidation_db() {
    match connect_cache_invalidation_db() {
        Ok(connection) => {
            match connection.execute_batch(
                &[
                    "BEGIN;".to_string(),
                    cache_invalidation(),
                    "COMMIT;".to_string(),
                ]
                .join("\n"),
            ) {
                Ok(_) => (),
                Err(error) => error!("Can't create cache_invalidation database: {}", error),
            }
        }
        Err(error) => error!(
            "Can't connect to the cache_invalidation database: {}",
            error
        ),
    }
}

fn cache_invalidation() -> String {
    r#"
        -- Create cache_invalidation
        CREATE TABLE IF NOT EXISTS cache_invalidation (
            version INTEGER PRIMARY KEY NOT NULL DEFAULT 1
        ) STRICT, WITHOUT ROWID;

        -- Create a trigger to automatically increment version
        CREATE TRIGGER IF NOT EXISTS auto_update_cache_invalidation
        BEFORE INSERT ON cache_invalidation
        WHEN (SELECT COUNT(*) FROM cache_invalidation) > 0
        BEGIN
            UPDATE cache_invalidation SET version = version + 1;
            
            SELECT RAISE(IGNORE);
        END;

        -- Create a trigger to prevent row deletion
        CREATE TRIGGER IF NOT EXISTS auto_avoid_delete_cache_invalidation
        BEFORE DELETE ON cache_invalidation
        BEGIN
            SELECT RAISE(FAIL, 'Deletion not allowed in cache_invalidation table');
        END;
        
        -- Create a trigger to prevent manual updates
        CREATE TRIGGER IF NOT EXISTS prevent_manual_update_cache_invalidation
        BEFORE UPDATE ON cache_invalidation
        BEGIN
            UPDATE cache_invalidation SET version = version + 1;
            
            SELECT RAISE(IGNORE);
        END;

        -- Initial insert to create the row (if needed)
        INSERT OR IGNORE INTO cache_invalidation DEFAULT VALUES;
    "#
    .to_string()
}
