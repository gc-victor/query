POST {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_token_list_1",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

POST {{host}}/_/user
Authorization: {{user_token}}
```json
{
    "email": "user_token_list_2",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

GET {{host}}/_/user/token
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[1].token" exists
jsonpath "$.data[1].token" != ""
jsonpath "$.data[1].write" == 1
jsonpath "$.data[1].expiration_date" exists
jsonpath "$.data[2].token" exists
jsonpath "$.data[2].token" != ""
jsonpath "$.data[2].write" == 1
jsonpath "$.data[2].expiration_date" exists

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
  "email": "user_token_list_1"
}
```
HTTP 200

DELETE {{host}}/_/user
Authorization: {{user_token}}
```json
{
  "email": "user_token_list_2"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "query": "DELETE FROM _config_user_token WHERE user_uuid IN ((SELECT uuid FROM _config_user WHERE email = 'user_token_list_1'), (SELECT uuid FROM _config_user WHERE email = 'user_token_list_2'))"
}
```
HTTP 200
