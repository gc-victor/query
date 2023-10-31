POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "INSERT INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'query_migration_is_admin',
        'password',
        1
    );"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_migration_is_admin');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE migration_is_admin (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/migration
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "
      INSERT INTO migration_is_admin (key, value) VALUES ('test-1', '1');
      INSERT INTO migration_is_admin (key, value) VALUES ('test-2', '2');
      INSERT INTO migration_is_admin (key, value) VALUES ('test-3', '3');
      INSERT INTO migration_is_admin (key, value) VALUES ('test-4', '4');
  "
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'query_migration_is_admin';"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "DROP TABLE migration_is_admin;"
}
```
HTTP 200
