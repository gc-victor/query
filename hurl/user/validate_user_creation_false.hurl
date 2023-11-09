POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "UPDATE _config_option SET value = '0' WHERE name = 'create_user';"
}
```
HTTP 200

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
HTTP 404

GET {{host}}/_/user
Authorization: {{user_token}}
HTTP 404

PUT {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_user_creation_accepted",
    "password": "password",
    "admin": false,
    "active": false
}
```
HTTP 404

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_validate_user_creation_accepted"
}
```
HTTP 404

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "UPDATE _config_option SET value = '1' WHERE name = 'create_user';"
}
```
HTTP 200