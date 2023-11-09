DELETE {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_value_name"
}
```
HTTP 200

# -----

POST {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_value_name"
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_value_name' AND active = 1 AND write = 1;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists


GET {{host}}/_/token/value?name=token_value_name
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_value_name"
}
```
HTTP 200