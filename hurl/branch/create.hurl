POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-create.sql",
  "query": "CREATE TABLE IF NOT EXISTS branch_create (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-create.sql",
  "branch_name": "branch_create"
}
```
HTTP 201

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-create.branch_create.branch.sql",
  "query": "INSERT INTO branch_create (key, value) VALUES ('test', '-');"
}
```
HTTP 200

DELETE {{host}}/_/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-create.branch_create.branch.sql"
}
```
HTTP 200