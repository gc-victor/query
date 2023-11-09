POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'user_list',
        'password',
        1
    );"
}
```
HTTP 200

GET {{host}}/_/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[1].email" == "user_list"
jsonpath "$.data[1].password" exists
jsonpath "$.data[1].admin" == 1
jsonpath "$.data[1].active" == 1

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'user_list';"
}
```
HTTP 200

