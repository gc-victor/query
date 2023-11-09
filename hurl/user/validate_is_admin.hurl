# It is implicit that the user_token is an admin token
POST {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_is_admin",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

GET {{host}}/_/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*].email" includes "user_validate_is_admin"
jsonpath "$.data[*].password" exists
jsonpath "$.data[*].admin" includes 1
jsonpath "$.data[*].active" includes 1

PUT {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_is_admin",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 200

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_is_admin"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_validate_is_admin';"
}
```
HTTP 200
