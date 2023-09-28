POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_expired_token",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

DELETE {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_expired_token"
}
```
HTTP 200

POST {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_expired_token",
    "expiration_date": {{prev_month}},
    "write": true
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_expired_token');"
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
HTTP 401

DELETE {{host}}/user
Authorization: {{user_token}}
```json
{
  "email": "query_user_token_validate_expired_token"
}
```
HTTP 200