POST http://localhost:3000/proxy
```json
{
    "key": "1",
    "value": "bun!"
}
```
HTTP 200

GET http://localhost:3000/proxy
HTTP 200
[Asserts]
jsonpath "$.[0].key" == "1"
jsonpath "$.[0].value" == "bun!"
