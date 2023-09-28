POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE user_token_create (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE user_token_create (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 400
[Asserts]
body contains "table user_token_create already exists in CREATE TABLE user_token_create (key TEXT NOT NULL UNIQUE, value TEXT); at offset 13"
