# It is implicit that the user_token is an admin token
POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_is_admin",
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
    "email": "user_token_validate_is_admin"
}
```
HTTP 200

GET {{host}}/user/token
Authorization: {{user_token}}
HTTP 200

POST {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_is_admin",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 201

PUT {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_is_admin",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 200

DELETE {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_is_admin"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_token_validate_is_admin';"
}
```
HTTP 200
