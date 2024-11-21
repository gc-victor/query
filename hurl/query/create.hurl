POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-query-create.sql",
  "query": "CREATE TABLE test (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data[0].changes" == 0
jsonpath "$.data[0].success" == true
