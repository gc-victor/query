# Migration

The migration endpoint allows to manage the migrations of the Query Server.

## POST

The migration endpoint allows to execute a migration in the primary database.

```http
POST /_/migration
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| db_name | string | The database to use. | true |
| query | string | The query to execute. | true |

Example:

```json
{
  "db_name": "example.sql",
  "query": "CREATE TABLE example (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
}
```
