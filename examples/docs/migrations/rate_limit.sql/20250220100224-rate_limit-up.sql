CREATE TABLE IF NOT EXISTS rate_limit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    request_count INTEGER DEFAULT 1,
    window_start INTEGER NOT NULL DEFAULT (strftime ('%s', 'now')),
    created_at INTEGER NOT NULL DEFAULT (strftime ('%s', 'now')),
    UNIQUE (ip_address, endpoint)
);

CREATE INDEX IF NOT EXISTS idx_rate_limit_lookup ON rate_limit (ip_address, endpoint, window_start);

CREATE INDEX IF NOT EXISTS idx_rate_limit_cleanup ON rate_limit (window_start);
