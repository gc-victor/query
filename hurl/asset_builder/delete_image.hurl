DELETE {{host}}/_/asset-builder
Authorization: {{user_token}}
```json
{
  "name": "dog.jpg"
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_asset.sql",
  "query": "SELECT * FROM asset WHERE name = 'dog.jpg';"
}
```
HTTP 200
[Asserts]
jsonpath "$.data" count == 0