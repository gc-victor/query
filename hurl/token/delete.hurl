# Delete Token 1
POST {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_1"
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_delete_name_1';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_1"
}
```
HTTP 200

# Delete Token 2

POST {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_2"
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_delete_name_2';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_2"
}
```
HTTP 200