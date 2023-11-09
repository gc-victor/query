POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
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
            'query_token_validate_write_false',
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"token\"}'),
            strftime('%s', datetime('now')),
            1,
            0
        );"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'query_token_validate_write_false';"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE query_token_validate_write_false (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_token_validate_write_false (key, value) VALUES ('test', '-');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE query_token_validate_write_false (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 400
[Asserts]
body contains "Token without write permission tried to write to the database"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_token_validate_write_false (key, value) VALUES ('test', '-');"
}
```
HTTP 400
[Asserts]
body contains "Token without write permission tried to write to the database"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "UPDATE query_token_validate_write_false SET value = '-';"
}
```
HTTP 400
[Asserts]
body contains "Token without write permission tried to write to the database"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "SELECT * FROM query_token_validate_write_false;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].key" contains "test"
jsonpath "$.data[0].value" contains "-"

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_token WHERE name = 'query_token_validate_write_false';"
}
```
HTTP 200