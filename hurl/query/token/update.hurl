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
            'query_token_update',
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"token\"}'),
            strftime('%s', datetime('now')),
            1,
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
  "query": "SELECT token FROM _config_token WHERE name = 'query_token_update';"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE query_token_update (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_token_update (key, value) VALUES ('test-1', '1');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_token_update (key, value) VALUES ('test-2', '2');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "UPDATE query_token_update SET value = 'updated';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].changes" == 2
jsonpath "$.data[0].success" == true

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "SELECT * FROM query_token_update;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].key" contains "test-1"
jsonpath "$.data[0].value" == "updated"
jsonpath "$.data[1].key" contains "test-2"
jsonpath "$.data[1].value" == "updated"

POST {{host}}/_/query
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "DROP TABLE query_token_update;"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_token WHERE name = 'query_token_update';"
}
```
HTTP 200