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
    "email": "query_login",
    "password": "password",
    "admin": true,
    "active": true
}
```
HTTP 201

POST {{host}}/_/function/api/admin/login
[FormParams]
email: without_password
HTTP 400
[Asserts]
body contains "{\"errors\":{\"password\":[\"Invalid type: Expected string but received null\"]}}"
cookie "session" not exists

POST {{host}}/_/function/api/admin/login
[FormParams]
password: without_email
HTTP 400
[Asserts]
body contains "{\"errors\":{\"email\":[\"Invalid type: Expected string but received null\"]}}"
cookie "session" not exists

POST {{host}}/_/function/api/admin/login
[FormParams]
email: query_login
password: wrong-password
HTTP 400
[Asserts]
body contains "The email or password is not correct."
cookie "session" not exists

POST {{host}}/_/function/api/admin/login
[FormParams]
email: admin
password: admin
HTTP 200
[Asserts]
header "Set-Cookie" contains "session="
header "Set-Cookie" contains "Path=/; Expires=3600000; Max-Age=3600000; HttpOnly; SameSite=Strict; Secure;"
jsonpath "$.data[0].token" exists

DELETE {{host}}/_/user
Authorization: Bearer {{user_token}}
```json
{
  "email": "query_login"
}
```
HTTP 200
