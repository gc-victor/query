POST {{host}}/_/query
Authorization: {{user_token}}
# Admin true
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user(
        email,
        password,
        admin
    ) VALUES (
        'query_user_token_validate_write_true',
        'password',
        1
    );"
}
```
HTTP 200

DELETE {{host}}/_/user/token
Authorization: {{user_token}}
```json
{
    "email": "query_user_token_validate_write_true"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT OR IGNORE INTO
    _config_user_token
        (
            user_uuid,
            token,
            expiration_date,
            write
        )
    VALUES
        (
            (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_write_true'),
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"user_token\"}'),
            strftime('%s', datetime('now')),
            1
    );"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'query_user_token_validate_write_true');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token;"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE test (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO test (key, value) VALUES ('test', '-');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'query_user_token_validate_write_true';"
}
```
HTTP 200