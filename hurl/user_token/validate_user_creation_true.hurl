POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "UPDATE _config_option SET value = '1' WHERE name = 'create_user';"
}
```
HTTP 200

POST {{host}}/_/user/token
Authorization: {{user_token}}
```json
{
    "email": "user_token_validate_user_creation",
    "expiration_date": {{next_month}},
    "write": true
}
```
HTTP 201

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
  "email": "user_token_validate_user_creation"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "query": "DELETE FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'user_token_validate_user_creation')"
}
```
HTTP 200