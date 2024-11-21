POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-query-insert.sql",
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
  "db_name": "hurl-query-insert.sql",
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
  "db_name": "hurl-query-insert.sql",
  "query": "INSERT INTO test (key, value) VALUES ('test1', 'test1');"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].rowid" == 2
jsonpath "$.data[0].success" == true