POST {{host}}/token
Authorization: {{user_token}}
```json
{
    "name": "token_list_name_1"
}
```
HTTP 201

POST {{host}}/token
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "name": "token_list_name_2",
    "active": false,
    "write": false
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "query": "SELECT token FROM _config_token WHERE name LIKE 'token_list_name%';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[1].token" exists

GET {{host}}/token
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].name" exists
jsonpath "$.data[0].name" == "token_list_name_1"
jsonpath "$.data[0].active" == 1
jsonpath "$.data[0].write" == 1
jsonpath "$.data[1].name" == "token_list_name_2"
jsonpath "$.data[1].active" == 0
jsonpath "$.data[1].write" == 0

DELETE {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_list_name_1"
}
```
HTTP 200

DELETE {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_list_name_2"
}
```
HTTP 200