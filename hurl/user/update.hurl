POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_update",
    "password": "password",
    "admin": false,
    "active": false
}
```
HTTP 201

GET {{host}}/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*].email" includes "user_update"
jsonpath "$.data[*].password" exists
jsonpath "$.data[*].admin" includes 0
jsonpath "$.data[*].active" includes 0

PUT {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_update",
    "password": "password-1",
    "admin": true,
    "active": true
}
```
HTTP 200

GET {{host}}/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*].email" includes "user_update"
jsonpath "$.data[*].password" exists
jsonpath "$.data[*].admin" includes 1
jsonpath "$.data[*].active" includes 1

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_update';"
}
```
HTTP 200