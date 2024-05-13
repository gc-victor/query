POST {{host}}/_/user/token/value
```json
{
    "email": "admin",
    "password": "admin"
}
```
HTTP 200
[Captures]
query_token: jsonpath "$.data[0].token"

POST {{host}}/_/function/api/admin/login
[FormParams]
email: admin
password: admin
HTTP 200
[Asserts]
header "Set-Cookie" contains "session="
[Captures]
session: cookie "session"

POST {{host}}/_/function/api/admin/token
[Cookies]
session: {{session}}
[MultipartFormData]
active: on
name: post
query_token: {{query_token}}
public_token: 6136673c-6585-48ac-9b96-d4351cbc1a1f
description: Test Post
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true

GET {{host}}/_/function/api/admin/token
[Cookies]
session: {{session}}
HTTP 200
[Captures]
token_uuid: jsonpath "$.data[0].uuid"
public_token: jsonpath "$.data[0].public_token"

POST {{host}}/_/function/api/post
Authorization: Bearer {{public_token}}
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

GET {{host}}/_/function/api/post
Authorization: Bearer {{public_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title"
jsonpath "$.data[0].content" == "Test content"
jsonpath "$.data[0].slug" == "/test-title"
jsonpath "$.data[0].image_url" == "/test.png"
[Captures]
uuid: jsonpath "$.data[0].uuid"

PUT {{host}}/_/function/api/post
Authorization: Bearer {{public_token}}
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

GET {{host}}/_/function/api/post
Authorization: Bearer {{public_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title 2"
jsonpath "$.data[0].content" == "Test content 2"
jsonpath "$.data[0].slug" == "/test-title-2"
jsonpath "$.data[0].image_url" == "/test2.png"

GET {{host}}/_/function/api/post/uuid/{{uuid}}
Authorization: Bearer {{public_token}}
HTTP 200
[Asserts]
jsonpath "$.data[0].title" == "Test Title 2"
jsonpath "$.data[0].content" == "Test content 2"
jsonpath "$.data[0].slug" == "/test-title-2"
jsonpath "$.data[0].image_url" == "/test2.png"

DELETE {{host}}/_/function/api/post
Authorization: Bearer {{public_token}}
```json
{
    "uuid": "{{uuid}}"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true

DELETE {{host}}/_/function/api/admin/token
[Cookies]
session: {{session}}
```json
{
    "uuid": "{{token_uuid}}"
}
```
HTTP 200
[Asserts]
jsonpath "$.data[0].success" == true
