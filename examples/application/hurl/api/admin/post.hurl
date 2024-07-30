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

POST {{host}}/_/function/api/admin/login
[FormParams]
email: admin
password: admin
HTTP 200
[Asserts]
header "Set-Cookie" contains "session="
[Captures]
session: cookie "session"

POST {{host}}/_/function/api/admin/post
[Cookies]
session: {{session}}
[MultipartFormData]
title: Test Title
content: Test content
slug: /test-title
image: file,post.jpg; image/jpeg
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

GET {{host}}/_/function/api/admin/post/uuid/{{uuid}}
[Cookies]
session: {{session}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title"
jsonpath "$.data[0].content" == "Test content"
jsonpath "$.data[0].slug" == "/test-title"
jsonpath "$.data[0].image_url" == "post/cache/post.jpg"
[Captures]
uuid: jsonpath "$.data[0].uuid"

PUT {{host}}/_/function/api/admin/post
[Cookies]
session: {{session}}
[MultipartFormData]
uuid: {{uuid}}
title: Test Title 2
content: Test content 2
slug: /test-title-2
image: file,post-2.jpg; image/jpeg
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true

GET {{host}}/_/function/api/admin/post/uuid/{{uuid}}
[Cookies]
session: {{session}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title 2"
jsonpath "$.data[0].content" == "Test content 2"
jsonpath "$.data[0].slug" == "/test-title-2"
jsonpath "$.data[0].image_url" == "post/cache/post-2.jpg"

DELETE {{host}}/_/function/api/admin/post
[Cookies]
session: {{session}}
```json
{
    "uuid": "{{uuid}}"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true
