POST {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_create",
    "password": "password",
    "admin": false,
    "active": false
}
```
HTTP 201

GET {{host}}/_/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*].email" includes "user_create"
jsonpath "$.data[*].password" exists
jsonpath "$.data[*].admin" includes 0
jsonpath "$.data[*].active" includes 0

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_create';"
}
```
HTTP 200
