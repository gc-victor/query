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

POST {{host}}/_/user
Authorization: Bearer {{user_token}}
```json
{
    "email": "query_studio_logout",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

POST {{host}}/_/function/api/admin/login
[FormParams]
email: query_studio_logout
password: password
HTTP 200
[Asserts]
header "Set-Cookie" contains "session="
header "Set-Cookie" contains "Path=/; Expires=3600000; Max-Age=3600000; HttpOnly; SameSite=Strict; Secure;"
jsonpath "$.data[0].token" exists
[Captures]
session: cookie "session"

GET {{host}}/_/function/api/admin/logout
[Cookies]
session: {{session}}
HTTP 302
[Asserts]
cookie "session" isEmpty
cookie "session[Max-Age]" == 0

DELETE {{host}}/_/user
Authorization: Bearer {{user_token}}
```json
{
  "email": "query_studio_logout"
}
```
HTTP 200
