POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "
    INSERT OR IGNORE INTO _config_user(
            email,
            password,
            admin
        ) VALUES (
            'migration_user_token_write_false',
            'password',
            1
        );
  "
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "
      UPDATE
        _config_user_token
      SET
        write = 0
      WHERE
        user_uuid = (SELECT uuid FROM _config_user WHERE email = 'migration_user_token_write_false');
    "
}
```
HTTP 200

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "SELECT token FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = 'migration_user_token_write_false');"
}
```
HTTP 200
[Captures]
token: jsonpath "$['data'][0]['token']"


POST {{host}}/migration
Authorization: Bearer {{token}}
```json
{
  "db_name": "hurl-test-query.sql",
  "query": "CREATE TABLE migration_user_token_write_false (key TEXT NOT NULL UNIQUE, value TEXT);"
}
```
HTTP 401

POST {{host}}/query
Authorization: {{user_token}}
```json
{
  "db_name": "config.sql",
  "query": "DELETE FROM _config_user WHERE email = 'migration_user_token_write_false';"
}
```
HTTP 200