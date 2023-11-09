POST {{host}}/_/user
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

DELETE {{host}}/_/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_expired_token"
}
```
HTTP 200

POST {{host}}/_/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_expired_token",
    "expiration_date": {{prev_month}},
    "write": true
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_expired_token');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token;"
}
```
HTTP 401

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
  "email": "query_user_token_validate_expired_token"
}
```
HTTP 200