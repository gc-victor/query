CREATE TABLE IF NOT EXISTS session(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session TEXT NOT NULL,
    token TEXT NOT NULL,
    expires_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TRIGGER IF NOT EXISTS trigger_session_update
    AFTER UPDATE ON session
    BEGIN
        UPDATE session
        SET updated_at=(strftime('%s', 'now'))
        WHERE id=OLD.id;
    END;
