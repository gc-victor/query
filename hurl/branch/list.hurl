POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-list.sql",
  "query": "CREATE TABLE branch_list (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 200

POST {{host}}/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-list.sql",
  "branch_name": "branch_list_1"
}
```
HTTP 201

POST {{host}}/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-list.sql",
  "branch_name": "branch_list_2"
}
```
HTTP 201

GET {{host}}/branch
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*]" count == 2
jsonpath "$.data[*].branch" includes "hurl-test-branch-list.branch_list_1.branch.sql"
jsonpath "$.data[*].branch" includes "hurl-test-branch-list.branch_list_2.branch.sql"

DELETE {{host}}/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-list.branch_list_1.branch.sql"
}
```
HTTP 200

DELETE {{host}}/branch
Authorization: {{user_token}}
```json
{
  "db_name": "hurl-test-branch-list.branch_list_2.branch.sql"
}
```
HTTP 200