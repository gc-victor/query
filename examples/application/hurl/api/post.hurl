POST {{host}}/_/user/token/value
```json
{
    "email": "admin",
    "password": "admin"
}
```
HTTP 200
[Captures]
user_token: jsonpath "$.data[0].token"

POST {{host}}/_/function/api/post
Authorization: Bearer {{user_token}}
```json
{
    "title": "Test Title",
    "content": "Test content",
    "slug": "/test-title",
    "image_url": "/test.png"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true

POST {{host}}/_/query
Authorization: Bearer {{user_token}}
```json
{
  "db_name": "post.sql",
  "query": "SELECT uuid FROM post WHERE slug = '/test-title'"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].uuid" exists
[Captures]
uuid: jsonpath "$.data[0].uuid"

GET {{host}}/_/function/api/post/uuid/{{uuid}}
Authorization: Bearer {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title"
jsonpath "$.data[0].content" == "Test content"
jsonpath "$.data[0].slug" == "/test-title"
jsonpath "$.data[0].image_url" == "/test.png"

PUT {{host}}/_/function/api/post
Authorization: Bearer {{user_token}}
```json
{
    "uuid": "{{uuid}}",
    "title": "Test Title 2",
    "content": "Test content 2",
    "slug": "/test-title-2",
    "image_url": "/test2.png"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true

GET {{host}}/_/function/api/post/uuid/{{uuid}}
Authorization: Bearer {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title 2"
jsonpath "$.data[0].content" == "Test content 2"
jsonpath "$.data[0].slug" == "/test-title-2"
jsonpath "$.data[0].image_url" == "/test2.png"

DELETE {{host}}/_/function/api/post
Authorization: Bearer {{user_token}}
```json
{
    "uuid": "{{uuid}}"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true
