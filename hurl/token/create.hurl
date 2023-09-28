# Create token with active = default (true) and write = default (true)
POST {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_create_name"
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_create_name' AND active = 1 AND write = 1;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

# Create token with active = false and write = false

POST {{host}}/token
Authorization: {{user_token}}
```json
{
  "name": "token_create_name_0_0",
  "active": false,
  "write": false
}
```
HTTP 201

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'token_create_name_0_0' AND active = 0 AND write = 0;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].token" exists

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_token WHERE name LIKE 'token_create_%';"
}
```
HTTP 200