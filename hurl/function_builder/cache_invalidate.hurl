# 1. Create two test functions
POST {{host}}/_/function-builder
Authorization: {{user_token}}
```json
{
    "function": [103,108,111,98,97,108,84,104,105,115,46,95,95,95,104,97,110,100,108,101,82,101,113,117,101,115,116,32,61,32,97,115,121,110,99,32,40,41,32,61,62,32,123,10,32,32,32,32,114,101,116,117,114,110,32,110,101,119,32,82,101,115,112,111,110,115,101,40,34,72,117,114,108,33,34,44,32,123,10,9,9,115,116,97,116,117,115,58,32,50,48,48,44,10,9,9,104,101,97,100,101,114,115,58,32,123,10,9,9,9,34,99,111,110,116,101,110,116,45,116,121,112,101,34,58,32,34,116,101,120,116,47,112,108,97,105,110,59,99,104,97,114,115,101,116,61,85,84,70,45,56,34,44,10,32,32,32,32,32,32,32,32,32,32,32,32,34,81,117,101,114,121,45,67,97,99,104,101,45,67,111,110,116,114,111,108,34,58,32,34,112,117,98,108,105,99,44,32,109,97,120,45,97,103,101,61,53,48,48,34,44,32,47,47,32,67,97,99,104,101,32,102,111,114,32,48,46,53,32,115,101,99,111,110,100,115,10,9,9,125,44,10,9,125,41,59,10,125],
    "method": "GET",
    "path": "/test1"
}
```

HTTP 200

POST {{host}}/_/function-builder
Authorization: {{user_token}}
```json
{
    "function": [103,108,111,98,97,108,84,104,105,115,46,95,95,95,104,97,110,100,108,101,82,101,113,117,101,115,116,32,61,32,97,115,121,110,99,32,40,41,32,61,62,32,123,10,32,32,32,32,114,101,116,117,114,110,32,110,101,119,32,82,101,115,112,111,110,115,101,40,34,72,117,114,108,33,34,44,32,123,10,9,9,115,116,97,116,117,115,58,32,50,48,48,44,10,9,9,104,101,97,100,101,114,115,58,32,123,10,9,9,9,34,99,111,110,116,101,110,116,45,116,121,112,101,34,58,32,34,116,101,120,116,47,112,108,97,105,110,59,99,104,97,114,115,101,116,61,85,84,70,45,56,34,44,10,32,32,32,32,32,32,32,32,32,32,32,32,34,81,117,101,114,121,45,67,97,99,104,101,45,67,111,110,116,114,111,108,34,58,32,34,112,117,98,108,105,99,44,32,109,97,120,45,97,103,101,61,53,48,48,34,44,32,47,47,32,67,97,99,104,101,32,102,111,114,32,48,46,53,32,115,101,99,111,110,100,115,10,9,9,125,44,10,9,125,41,59,10,125],
    "method": "GET",
    "path": "/test2"
}
```

HTTP 200

# 2. First request - should not be cached
GET {{host}}/_/function/test1
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists
body contains "Hurl!"

GET {{host}}/_/function/test2
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists
body contains "Hurl!"

# 3. Second request - should be cached
GET {{host}}/_/function/test1
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"
body contains "Hurl!"

GET {{host}}/_/function/test2
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"
body contains "Hurl!"

# 4. Clear the cache by deleting first function
DELETE {{host}}/_/function-builder
Authorization: {{user_token}}
```json
{
    "method": "GET",
    "path": "/test1"
}
```

HTTP 200

# 5. Request second function after cache clear - should not be cached
GET {{host}}/_/function/test2
[Options]
delay: 500
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists
body contains "Hurl!"

# 6. One more request - should be cached again
GET {{host}}/_/function/test2
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"
body contains "Hurl!"

# 7. Create new function
POST {{host}}/_/function-builder
Authorization: {{user_token}}
```json
{
    "function": [103,108,111,98,97,108,84,104,105,115,46,95,95,95,104,97,110,100,108,101,82,101,113,117,101,115,116,32,61,32,97,115,121,110,99,32,40,41,32,61,62,32,123,10,32,32,32,32,114,101,116,117,114,110,32,110,101,119,32,82,101,115,112,111,110,115,101,40,34,72,117,114,108,33,34,44,32,123,10,9,9,115,116,97,116,117,115,58,32,50,48,48,44,10,9,9,104,101,97,100,101,114,115,58,32,123,10,9,9,9,34,99,111,110,116,101,110,116,45,116,121,112,101,34,58,32,34,116,101,120,116,47,112,108,97,105,110,59,99,104,97,114,115,101,116,61,85,84,70,45,56,34,44,10,32,32,32,32,32,32,32,32,32,32,32,32,34,81,117,101,114,121,45,67,97,99,104,101,45,67,111,110,116,114,111,108,34,58,32,34,112,117,98,108,105,99,44,32,109,97,120,45,97,103,101,61,53,48,48,34,44,32,47,47,32,67,97,99,104,101,32,102,111,114,32,48,46,53,32,115,101,99,111,110,100,115,10,9,9,125,44,10,9,125,41,59,10,125],
    "method": "GET",
    "path": "/test1"
}
```

HTTP 200

# 8. First request to new function (test1) - should not be cached
GET {{host}}/_/function/test1
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists
body contains "Hurl!"

# 9. Second request (test1) - should be cached
GET {{host}}/_/function/test1
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"
body contains "Hurl!"

# 10. First request to new function (test2) - should not be cached
GET {{host}}/_/function/test2
[Options]
delay: 500
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists
body contains "Hurl!"

# 11. Second request (test2) - should be cached
GET {{host}}/_/function/test2
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"
body contains "Hurl!"