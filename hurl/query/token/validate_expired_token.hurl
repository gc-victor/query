POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "INSERT INTO
        _config_token(
            name,
            token,
            expiration_date,
            active,
            write
        )
    VALUES
        (
            'query_token_validate_expired_token',
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now', '-1 month')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"token\"}'),
            strftime('%s', datetime('now', '-1 month')),
            1,
            1
        );"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'query_token_validate_expired_token';"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_token;"
}
```
HTTP 401

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_token WHERE name = 'query_token_validate_expired_token';"
}
```
HTTP 200