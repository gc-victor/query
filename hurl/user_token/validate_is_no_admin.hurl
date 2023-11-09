POST {{host}}/_/query
Authorization: {{user_token}}
# Admin false
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'user_token_validate_is_no_admin',
        'password',
        0
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
            (SELECT uuid FROM _config_user WHERE email = 'user_token_validate_is_no_admin'),
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
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_validate_is_no_admin');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

GET {{host}}/_/user/token
Authorization: Bearer {{token}}
HTTP 401

POST {{host}}/_/user/token
Authorization: Bearer {{token}}
```json
{
    "email": "user_token_validate_is_no_admin",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 401

PUT {{host}}/_/user/token
Authorization: Bearer {{token}}
```json
{
    "email": "user_token_validate_is_no_admin",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 401

DELETE {{host}}/_/user/token
Authorization: Bearer {{token}}
```json
{
    "email": "user_token_validate_is_no_admin"
}
```
HTTP 401

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_token_validate_is_no_admin';"
}
```
HTTP 200