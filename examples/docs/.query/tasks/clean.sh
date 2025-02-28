#!/bin/sh

rm -rf .query/.cache &
rm -rf dist &
rm -rf .dbs/query_asset.sql &
rm -rf .dbs/query_function.sql &
rm -rf .dbs/query_cache_function.sql &
echo "Done!"
