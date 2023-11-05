POST {{host}}/query
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
        'query_user_token_validate_write_false',
        'password',
        1
    );"
}
```
HTTP 200

DELETE {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_write_false"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user_token
        (
            user_uuid,
            token,
            expiration_date,
            write
        )
    VALUES
        (
            (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_write_false'),
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"user_token\"}'),
            strftime('%s', datetime('now')),
            0
    );"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_write_false');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token;"
}
```
HTTP 200

POST {{host}}/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE test (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 400
[Asserts]
body contains "Token without write permission tried to write to the database"

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'query_user_token_validate_write_false';"
}
```
HTTP 200