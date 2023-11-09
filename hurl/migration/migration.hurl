POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE migration_migration (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/migration
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "
      INSERT INTO migration_migration (key, value) VALUES ('test-1', '1');
      INSERT INTO migration_migration (key, value) VALUES ('test-2', '2');
      INSERT INTO migration_migration (key, value) VALUES ('test-3', '3');
      INSERT INTO migration_migration (key, value) VALUES ('test-4', '4');
  "
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "SELECT * FROM migration_migration;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].key" contains "test-1"
jsonpath "$.data[0].value" contains "1"
jsonpath "$.data[1].key" contains "test-2"
jsonpath "$.data[1].value" contains "2"
jsonpath "$.data[2].key" contains "test-3"
jsonpath "$.data[2].value" contains "3"
jsonpath "$.data[3].key" contains "test-4"
jsonpath "$.data[3].value" contains "4"

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "DROP TABLE migration_migration;"
}
```
HTTP 200
