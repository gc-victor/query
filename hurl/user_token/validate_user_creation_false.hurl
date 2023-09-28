POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "UPDATE _config_option SET value = '0' WHERE name = 'create_user';"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'user_token_validate_user_creation',
        'password',
        1
    );"
}
```
HTTP 200

POST {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_user_creation",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 404

GET {{host}}/user/token
Authorization: {{user_token}}
HTTP 404

PUT {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_user_creation",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 404

DELETE {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_user_creation"
}
```
HTTP 404

POST {{host}}/query
Authorization: {{user_token}}
```json
{
    "db_name": "config.sql",
    "query": "DELETE FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_validate_user_creation')"
}
```
HTTP 200