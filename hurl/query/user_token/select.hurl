POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE user_token_select (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO user_token_select (key, value) VALUES ('test-1', '1');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO user_token_select (key, value) VALUES ('test-2', '2');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "SELECT * FROM user_token_select;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].key" contains "test-1"
jsonpath "$.data[0].value" contains "1"
jsonpath "$.data[1].key" contains "test-2"
jsonpath "$.data[1].value" contains "2"
