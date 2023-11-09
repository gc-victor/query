POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE query_user_token_get (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

GET {{host}}/_/query?db_name=hurl-test-query.sql&query=INSERT%20INTO%20query_user_token_get%20(key,%20value)%20VALUES%20('test-1',%20'1');
Authorization: {{user_token}}
HTTP 400
[Asserts]
body contains "GET requests only allows read queries"

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_user_token_get (key, value) VALUES ('test-1', '1');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_user_token_get (key, value) VALUES ('test-2', '2');"
}
```
HTTP 200

GET {{host}}/_/query?db_name=hurl-test-query.sql&query=SELECT%20*%20FROM%20query_user_token_get;
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].key" contains "test-1"
jsonpath "$.data[0].value" contains "1"
jsonpath "$.data[1].key" contains "test-2"
jsonpath "$.data[1].value" contains "2"

GET {{host}}/_/query
Authorization: {{user_token}}
HTTP 400
[Asserts]
body contains "Missing query string"

GET {{host}}/_/query?
Authorization: {{user_token}}
HTTP 400
[Asserts]
body contains "Missing query string: db_name"

GET {{host}}/_/query?db_name=hurl-test-query.sql
Authorization: {{user_token}}
HTTP 400
[Asserts]
body contains "Missing query string: query"