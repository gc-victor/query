POST {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_update_name"
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_update_name' AND active = 1 AND write = 1;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

PUT {{host}}/token
Authorization: {{user_token}}
```json
{
    "name": "token_update_name",
    "active": false
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_update_name' AND active = 0 AND write = 1;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

PUT {{host}}/token
Authorization: {{user_token}}
```json
{
    "name": "token_update_name",
    "write": false
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_update_name' AND active = 0 AND write = 0;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_token WHERE name LIKE 'token_update_name';"
}
```
HTTP 200