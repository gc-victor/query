DELETE {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_value_name"
}
```
HTTP 200

# -----

POST {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_value_name"
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_value_name' AND active = 1 AND write = 1;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists


GET {{host}}/token/value?name=token_value_name
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_value_name"
}
```
HTTP 200