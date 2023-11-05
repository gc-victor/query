POST {{host}}/query
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
            'query_token_get',
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"token\"}'),
            strftime('%s', datetime('now')),
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
  "db_name": "query_config.sql",
  "query": "SELECT token FROM _config_token WHERE name = 'query_token_get';"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE query_token_get (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_token_get (key, value) VALUES ('test-1', '1');"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_token_get (key, value) VALUES ('test-2', '2');"
}
```
HTTP 200

GET {{host}}/query?db_name=hurl-test-query.sql&query=INSERT%20INTO%20query_user_token_get%20(key,%20value)%20VALUES%20('test-1',%20'1');
Authorization: Bearer {{token}}
HTTP 400
[Asserts]
body contains "GET requests only allows read queries"

GET {{host}}/query?db_name=hurl-test-query.sql&query=SELECT%20*%20FROM%20query_token_get;
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].key" contains "test-1"
jsonpath "$.data[0].value" contains "1"
jsonpath "$.data[1].key" contains "test-2"
jsonpath "$.data[1].value" contains "2"

GET {{host}}/query
Authorization: Bearer {{token}}
HTTP 400
[Asserts]
body contains "Missing query string"

GET {{host}}/query?
Authorization: Bearer {{token}}
HTTP 400
[Asserts]
body contains "Missing query string: db_name"

GET {{host}}/query?db_name=hurl-test-query.sql
Authorization: Bearer {{token}}
HTTP 400
[Asserts]
body contains "Missing query string: query"

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_token WHERE name = 'query_token_get';"
}
```
HTTP 200