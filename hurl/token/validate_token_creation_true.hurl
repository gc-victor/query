POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "UPDATE _config_option SET value = '1' WHERE name = 'create_token';"
}
```
HTTP 200

POST {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "validate_token_creation"
}
```
HTTP 201

GET {{host}}/_/token
Authorization: {{user_token}}
HTTP 200

PUT {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "validate_token_creation"
}
```
HTTP 200

DELETE {{host}}/_/token
Authorization: {{user_token}}
```json
{
  "name": "validate_token_creation"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_token WHERE name LIKE 'validate_token_creation';"
}
```
HTTP 200