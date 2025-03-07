-- Drop the trigger first
DROP TRIGGER IF EXISTS update_docs_search;

-- Drop the virtual FTS5 table
DROP TABLE IF EXISTS docs_search;