POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_token_create",
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
    "email": "user_token_create"
}
```
HTTP 200

POST {{host}}/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_create",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
    "db_name": "config.sql",
    "query": "SELECT * FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_create')"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists
jsonpath "$.data[0].token" != ""
jsonpath "$.data[0].write" == 1
jsonpath "$.data[0].expiration_date" == {{next_month}}

DELETE {{host}}/user
Authorization: {{user_token}}
```json
{
  "email": "user_token_create"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
    "db_name": "config.sql",
    "query": "DELETE FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_create')"
}
```
HTTP 200