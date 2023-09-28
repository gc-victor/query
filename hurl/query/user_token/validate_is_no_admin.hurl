POST {{host}}/query
Authorization: {{user_token}}
# Admin false
```json
{
  "db_name": "config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'query_user_token_validate_no_admin',
        'password',
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
            (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_no_admin'),
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"user_token\"}'),
            strftime('%s', datetime('now')),
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
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_no_admin');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token;"
}
```
HTTP 400
[Asserts]
body contains "Can't query the config database without being admin"

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'query_user_token_validate_no_admin';"
}
```
HTTP 200