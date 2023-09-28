# Delete Token 1
POST {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_1"
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_delete_name_1';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_1"
}
```
HTTP 200

# Delete Token 2

POST {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_2"
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_delete_name_2';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

DELETE {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_delete_name_2"
}
```
HTTP 200