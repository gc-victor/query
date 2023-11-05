POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "UPDATE _config_option SET value = '1' WHERE name = 'create_user';"
}
```
HTTP 200

POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_user_creation_accepted",
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
jsonpath "$.data[*].email" includes "user_validate_user_creation_accepted"
jsonpath "$.data[*].password" exists
jsonpath "$.data[*].admin" includes 0
jsonpath "$.data[*].active" includes 0

PUT {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_user_creation_accepted",
    "password": "password",
    "admin": false,
    "active": false
}
```
HTTP 200

DELETE {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_user_creation_accepted"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_validate_user_creation_accepted';"
}
```
HTTP 200

