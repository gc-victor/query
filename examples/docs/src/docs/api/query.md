# Query

The `query` endpoint allows to execute queries in the databases. Using the `GET` method, the query is executed in the database closest to the user's region, thanks to the LiteFS proxy. Using the `POST` method, the query is executed in the primary database.

## POST

The `query` endpoint allows to execute a query in the primary database.

```http
POST /_/query
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
| params | object \| array | The params to use in the query. | false |

The params object should use kyes with the format ":AAA", "$AAA", or "@AAA" that serve as placeholders for values that are bound to the parameters at a later time.

Example:

```json
{
  "db_name": "example.sql",
  "query": "SELECT * FROM example WHERE id = :id",
  "params": {
    ":id": 1
  }
}
```

In the case of the array, the values are bound to the parameters based on the order of the array.

Example:

```json
{
  "db_name": "example.sql",
  "query": "SELECT * FROM example WHERE id = ?",
  "params": [1]
}
```

## GET

By using the `GET` method, data can be retrieved with less latency from the database closest to the user's region, thanks to the LiteFS proxy.

```http
GET /_/query?db_name=<DB_NAME>&query=<QUERY>&params=<PARAMS>
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Query String

| Name | Type | Format | Description | Required |
| :--- | :--- | :--- | :--- | :--- |
| db_name | string | - | The database to use. | true |
| query | string | URL Encoded | The SELECT query. | true |
| params | object \| array | URL Encoded | The params to use in the query. | false |

Example:

```http
GET /_/query?db_name=example.sql&query=SELECT%20*%20FROM%20example%20WHERE%20id%20%3D%20%3F&params=%5B1%5D
```
