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
        'token_validate_is_admin',
        'password',
        1
    );"
}
```
HTTP 200

POST {{host}}/_/query
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
            (SELECT uuid FROM _config_user WHERE email = 'token_validate_is_admin'),
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"user_token\"}'),
            strftime('%s', datetime('now')),
            1
    );"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'token_validate_is_admin');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/token
Authorization: Bearer {{token}}
```json
{
  "name": "token_validate_is_admin"
}
```
HTTP 201

GET {{host}}/_/token
Authorization: Bearer {{token}}
HTTP 200

PUT {{host}}/_/token
Authorization: Bearer {{token}}
```json
{
    "name": "token_validate_is_admin",
    "active": false
}
```
HTTP 200

DELETE {{host}}/_/token
Authorization: Bearer {{token}}
```json
{
  "name": "token_validate_is_admin"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'token_validate_is_admin';"
}
```
HTTP 200