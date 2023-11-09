POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "INSERT INTO
        _config_token(
            name,
            token,
            expiration_date,
            active,
            write
        )
    VALUES
        (
            :name,
            token('{\"sub\": \"' || (SELECT uuid()) ||  '\", \"exp\": ' || strftime('%s', datetime('now')) || ', \"iat\": ' || strftime('%s', datetime('now')) || ', \"iss\": \"token\"}'),
            strftime('%s', datetime('now')),
            :active,
            1
        );",
    "params": {
        ":name": "query_user_token_params",
        ":active": 1
    }
}
```
HTTP 200

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
    "db_name": "query_config.sql",
    "query": "SELECT token FROM _config_token WHERE name = :name AND active = 1;",
    "params": ["query_user_token_params"]
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

# JSON params
GET {{host}}/_/query?db_name=query_config.sql&query=SELECT%20token%20FROM%20_config_token%20WHERE%20name%20=%20:name%20AND%20active%20=%201;&params=%7B%22:name%22:%22query_user_token_params%22%7D
Authorization: {{user_token}}
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

# Array params
GET {{host}}/_/query?db_name=query_config.sql&query=SELECT%20token%20FROM%20_config_token%20WHERE%20name%20=%20?%20AND%20active%20=%201;&params=%5B%22query_user_token_params%22%5D
Authorization: {{user_token}}
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"

POST {{host}}/_/query
Authorization: {{user_token}}
```json
{
  "db_name": "query_config.sql",
  "query": "DELETE FROM _config_token WHERE name = 'query_user_token_params';"
}
```
HTTP 200