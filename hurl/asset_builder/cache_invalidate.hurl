# 1. First create a test asset
POST {{host}}/_/asset-builder
Authorization: {{user_token}}
```json
{
    "active": true,
    "data": [116,101,115,116],
    "file_hash": "123",
    "mime_type": "text/plain",
    "name": "cache-test.txt"
}
```

POST {{host}}/_/asset-builder
Authorization: {{user_token}}
```json
{
    "active": true,
    "data": [116,101,115,116],
    "file_hash": "123",
    "mime_type": "text/plain",
    "name": "cache-test-2.txt"
}
```

HTTP 200

# 2. First request - should not be cached
GET {{host}}/_/asset/cache-test.txt
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists

# 3. Second request - should be cached
GET {{host}}/_/asset/cache-test.txt
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"

# 4. Clear the cache
DELETE {{host}}/_/asset-builder
Authorization: {{user_token}}
```json
{
    "name": "cache-test.txt"
}
```

HTTP 200

# 5. Request after cache clear - should not be cached
GET {{host}}/_/asset/cache-test-2.txt
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists

# 6. One more request - should be cached again
GET {{host}}/_/asset/cache-test-2.txt
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"

# 7. POST a new asset
POST {{host}}/_/asset-builder
Authorization: {{user_token}}
```json
{
    "active": true,
    "data": [116,101,115,116],
    "file_hash": "123",
    "mime_type": "text/plain",
    "name": "cache-test.txt"
}
```

HTTP 200

# 8. First request - should not be cached
GET {{host}}/_/asset/cache-test.txt
[Options]
delay: 5100
HTTP 200
[Asserts]
header "Query-Cache-Hit" not exists

# 9. Second request - should be cached
GET {{host}}/_/asset/cache-test.txt
HTTP 200
[Asserts]
header "Query-Cache-Hit" == "true"
