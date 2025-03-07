-- First create the virtual table if it doesn't exist
CREATE VIRTUAL TABLE IF NOT EXISTS docs_search USING fts5 (
    title,
    path,
    plain_text,
    tokenize = 'porter unicode61',
    prefix = '2 3'
);

-- Clear existing data if needed
DELETE FROM docs_search;

-- Insert data extracted from JSON
INSERT INTO
    docs_search (title, path, plain_text)
SELECT
    json_extract (data, '$.title') AS title,
    json_extract (data, '$.path') AS path,
    json_extract (data, '$.plain_text') AS plain_text
FROM
    asset
WHERE
    name LIKE 'dist/docs/%.json';

-- Create a trigger to update the search index after insert
CREATE TRIGGER IF NOT EXISTS update_docs_search_insert AFTER INSERT ON asset WHEN NEW.name LIKE 'dist/docs/%.json' BEGIN
-- Remove the old version if it exists
DELETE FROM docs_search
WHERE
    path = json_extract (NEW.data, '$.path');

-- Insert the new/updated document
INSERT INTO
    docs_search (title, path, plain_text)
VALUES
    (
        json_extract (NEW.data, '$.title'),
        json_extract (NEW.data, '$.path'),
        json_extract (NEW.data, '$.plain_text')
    );

END;

-- Create a trigger to update the search index after update
CREATE TRIGGER IF NOT EXISTS update_docs_search_update AFTER
UPDATE ON asset WHEN NEW.name LIKE 'dist/docs/%.json' BEGIN
-- Remove the old version if it exists
DELETE FROM docs_search
WHERE
    path = json_extract (NEW.data, '$.path');

-- Insert the new/updated document
INSERT INTO
    docs_search (title, path, plain_text)
VALUES
    (
        json_extract (NEW.data, '$.title'),
        json_extract (NEW.data, '$.path'),
        json_extract (NEW.data, '$.plain_text')
    );

END;
