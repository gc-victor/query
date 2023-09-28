POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE user_token_update (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO user_token_update (key, value) VALUES ('test-1', '1');"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "INSERT INTO user_token_update (key, value) VALUES ('test-2', '2');"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "UPDATE user_token_update SET value = 'updated' WHERE key = 'test-1';"
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "SELECT * FROM user_token_update;"
}
```
HTTP 200
{"data":[{"key":"test-1","value":"updated"},{"key":"test-2","value":"2"}]}