POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-delete.sql",
  "query": "CREATE TABLE branch_delete (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/_/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-delete.sql",
  "branch_name": "branch_delete"
}
```
HTTP 201

DELETE {{host}}/_/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-delete.branch_delete.branch.sql"
}
```
HTTP 200