POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE migration_migration (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/migration
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "
      INSERT INTO migration_migration (key, value) VALUES ('test-1', '-');
      INSERT INTO migration_migration (key, value) VALUES ('test-2', '-');
      INSERT INTO migration_migration (key, value) VALUES ('test-3', '-');
      INSERT INTO migration_migration (key, value) VALUES ('test-4', '-');
  "
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "SELECT * FROM migration_migration;"
}
```
HTTP 200
{"data":[{"key":"test-1","value":"-"},{"key":"test-2","value":"-"},{"key":"test-3","value":"-"},{"key":"test-4","value":"-"}]}

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "DROP TABLE migration_migration;"
}
```
HTTP 200
