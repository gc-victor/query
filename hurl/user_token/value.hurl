POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_token_value",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

POST {{host}}/user/token/value
```json
{
    "email": "user_token_value",
    "password": "password"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

GET {{host}}/user/token/value?email=user_token_value
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/user
Authorization: {{user_token}}
```json
{
  "email": "user_token_value"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
    "db_name": "config.sql",
    "query": "DELETE FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_value')"
}
```
HTTP 200
