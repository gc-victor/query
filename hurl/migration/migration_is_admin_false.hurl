POST {{host}}/_/query
Authorization: {{user_token}}
# Admin true
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'query_migration_is_admin_false',
        'password',
        0
    );"
}
```
HTTP 200

PUT {{host}}/_/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_migration_is_admin_false",
    "write": false
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_migration_is_admin_false');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/migration
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE migration_is_admin_false (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 401

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'query_migration_is_admin_false';"
}
```
HTTP 200
