POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-query-select.sql",
  "query": "CREATE TABLE test (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].changes" == 0
jsonpath "$.data[0].success" == true

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-query-select.sql",
  "query": "INSERT INTO test (key, value) VALUES ('test', 'test');"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].rowid" == 1
jsonpath "$.data[0].success" == true

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-query-select.sql",
  "query": "INSERT INTO test (key, value) VALUES ('test1', 'test1');"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].rowid" == 2
jsonpath "$.data[0].success" == true

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-query-select.sql",
  "query": "SELECT * FROM test;"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].key" == "test"
jsonpath "$.data[0].value" == "test"
jsonpath "$.data[1]" exists
jsonpath "$.data[1].key" == "test1"
jsonpath "$.data[1].value" == "test1"