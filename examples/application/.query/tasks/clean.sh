#!/bin/sh

rm -rf .query/.cache &
rm -rf dist &
sqlite3 .dbs/query_asset.sql "DELETE from asset" &
sqlite3 .dbs/query_function.sql "DELETE from function" &
echo "Done!"
