-- Create the counter table with single row constraint
CREATE TABLE IF NOT EXISTS counter (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    value INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial counter value
INSERT
OR IGNORE INTO counter (id, value)
VALUES
    (1, 0);

-- Create trigger to update on insert
CREATE TRIGGER IF NOT EXISTS insert_to_update_counter BEFORE INSERT ON counter BEGIN
UPDATE counter
SET
    value = NEW.value
WHERE
    id = 1;

END;

-- Create trigger to update the timestamp
CREATE TRIGGER IF NOT EXISTS update_counter_timestamp AFTER
UPDATE ON counter BEGIN
UPDATE counter
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = 1;

END;
