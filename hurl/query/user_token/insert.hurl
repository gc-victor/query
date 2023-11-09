POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE query_user_token_insert (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_user_token_insert (key, value) VALUES ('test', '-');"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO query_user_token_insert (key, value) VALUES ('test', '-');"
}
```
HTTP 400
[Asserts]
body contains "UNIQUE constraint failed: query_user_token_insert.key"