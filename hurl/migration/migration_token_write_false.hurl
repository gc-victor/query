POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "INSERT INTO
        _config_token(
            name,
            token,
            expiration_date,
            active,
            write
        )
    VALUES
        (
            'migration_token_write_false',
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"user_token\"}'),
            strftime('%s', datetime('now')),
            1,
            0
        );"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'migration_token_write_false';"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/migration
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE migration_token_write_false (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 401

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_token WHERE name = 'migration_token_write_false';"
}
```
HTTP 200