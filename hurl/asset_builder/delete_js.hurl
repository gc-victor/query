DELETE {{host}}/_/asset-builder
Authorization: {{user_token}}
```json
{
  "name": "test.js"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_asset.sql",
  "query": "SELECT * FROM asset WHERE name = 'test.js';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" count == 0