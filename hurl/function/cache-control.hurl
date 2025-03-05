POST {{host}}/_/function-builder
Authorization: {{user_token}}
```json
{
    "method": "GET",
    "path": "/hurl/cache-control",
    "function": [103,108,111,98,97,108,84,104,105,115,46,95,95,95,104,97,110,100,108,101,82,101,113,117,101,115,116,32,61,32,97,115,121,110,99,32,40,41,32,61,62,32,123,10,32,32,32,32,114,101,116,117,114,110,32,110,101,119,32,82,101,115,112,111,110,115,101,40,34,72,117,114,108,33,34,44,32,123,10,9,9,115,116,97,116,117,115,58,32,50,48,48,44,10,9,9,104,101,97,100,101,114,115,58,32,123,10,9,9,9,34,99,111,110,116,101,110,116,45,116,121,112,101,34,58,32,34,116,101,120,116,47,112,108,97,105,110,59,99,104,97,114,115,101,116,61,85,84,70,45,56,34,44,10,32,32,32,32,32,32,32,32,32,32,32,32,34,81,117,101,114,121,45,67,97,99,104,101,45,67,111,110,116,114,111,108,34,58,32,34,112,117,98,108,105,99,44,32,109,97,120,45,97,103,101,61,53,48,48,34,44,32,47,47,32,67,97,99,104,101,32,102,111,114,32,48,46,53,32,115,101,99,111,110,100,115,10,9,9,125,44,10,9,125,41,59,10,125]

}
```
HTTP 200

# First request
GET {{host}}/_/function/hurl/cache-control
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" not exists

# The cache should be hit
GET {{host}}/_/function/hurl/cache-control
Authorization: {{user_token}}
[Options]
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" exists
header "Query-Cache-Hit" contains "true"

# The cache should be hit
GET {{host}}/_/function/hurl/cache-control
Authorization: {{user_token}}
[Options]
delay: 50
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" exists
header "Query-Cache-Hit" contains "true"

# The cache should be expired
GET {{host}}/_/function/hurl/cache-control
Authorization: {{user_token}}
[Options]
delay: 550
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" not exists

POST {{host}}/_/function-builder
Authorization: {{user_token}}
```json
{
    "method": "GET",
    "path": "/hurl/cache-control-query",
    "function": [103,108,111,98,97,108,84,104,105,115,46,95,95,95,104,97,110,100,108,101,82,101,113,117,101,115,116,32,61,32,97,115,121,110,99,32,40,41,32,61,62,32,123,10,32,32,32,32,114,101,116,117,114,110,32,110,101,119,32,82,101,115,112,111,110,115,101,40,34,72,117,114,108,33,34,44,32,123,10,9,9,115,116,97,116,117,115,58,32,50,48,48,44,10,9,9,104,101,97,100,101,114,115,58,32,123,10,9,9,9,34,99,111,110,116,101,110,116,45,116,121,112,101,34,58,32,34,116,101,120,116,47,112,108,97,105,110,59,99,104,97,114,115,101,116,61,85,84,70,45,56,34,44,10,32,32,32,32,32,32,32,32,32,32,32,32,34,81,117,101,114,121,45,67,97,99,104,101,45,67,111,110,116,114,111,108,34,58,32,34,112,117,98,108,105,99,44,32,109,97,120,45,97,103,101,61,53,48,48,34,44,32,47,47,32,67,97,99,104,101,32,102,111,114,32,48,46,53,32,115,101,99,111,110,100,115,10,9,9,125,44,10,9,125,41,59,10,125]

}
```
HTTP 200

# Test with a single query parameter
GET {{host}}/_/function/hurl/cache-control-query?param=value
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" not exists

# The cache should be hit with the same query parameter
GET {{host}}/_/function/hurl/cache-control-query?param=value
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" exists
header "Query-Cache-Hit" contains "true"

# Different query parameter should not hit the cache
GET {{host}}/_/function/hurl/cache-control-query?param=different
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" not exists

# Multiple query parameters
GET {{host}}/_/function/hurl/cache-control?first=value1&second=value2
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" not exists

# Same multiple query parameters should hit the cache
GET {{host}}/_/function/hurl/cache-control?first=value1&second=value2
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" exists
header "Query-Cache-Hit" contains "true"

# Different order of query parameters should still hit the cache
GET {{host}}/_/function/hurl/cache-control?second=value2&first=value1
Authorization: {{user_token}}
HTTP 200
[Asserts]
body contains "Hurl!"
header "Query-Cache-Control" contains "max-age=500"
header "Query-Cache-Hit" not exists
