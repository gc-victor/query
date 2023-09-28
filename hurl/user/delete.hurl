POST {{host}}/user
Authorization: {{user_token}}
```json
{
    "email": "user_delete",
    "password": "password",
    "admin": false,
    "active": false
}
```
HTTP 201

GET {{host}}/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*].email" includes "user_delete"

DELETE {{host}}/user
Authorization: {{user_token}}
```json
{
  "email": "user_delete"
}
```
HTTP 200

GET {{host}}/user
Authorization: {{user_token}}
HTTP 200
[Asserts]
jsonpath "$.data[*].email" not includes "user_delete"
