POST {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_token_delete",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "query": "SELECT * FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_delete')"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists
jsonpath "$.data[0].token" != ""
jsonpath "$.data[0].write" == 1
jsonpath "$.data[0].expiration_date" exists

DELETE {{host}}/_/user/token
Authorization: {{user_token}}
```json
{
  "email": "user_token_delete"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "query": "SELECT * FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_delete')"
}
```
HTTP 200
{"data":[]}

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
  "email": "user_token_delete"
}
```
HTTP 200