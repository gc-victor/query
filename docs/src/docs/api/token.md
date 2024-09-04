# Token

The token endpoint allows to manage the tokens not related to a user.

## POST

The token endpoint allows to create a new token.

```http
POST /_/token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| name | string | The name of the token. | - | true |
| expiration_date | number | The expiration date in milliseconds. | updated_at | false |
| active | boolean | If the token is active | true | false |
| write | boolean | If the token has write permissions. | true | false |

Example:

```json
{
  "name": "example",
  "expiration_date": 1632960000000,
  "active": true,
  "write": true
}
```

## GET

The token endpoint allows to get a list of all the tokens.

```http
GET /_/token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

## PUT

The token endpoint allows to update a token.

```http
PUT /_/token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| name | string | The name of the token. | - | true |
| expiration_date | number | The expiration date in milliseconds. | updated_at | false |
| active | boolean | If the token is active | true | false |
| write | boolean | If the token has write permissions. | true | false |

Example:

```json
{
  "name": "example",
  "expiration_date": 1632960000000,
  "active": true,
  "write": true
}
```

## DELETE

The token endpoint allows to delete a token.

```http
DELETE /_/token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| name | string | The name of the token. | true |

## GET Value

The token endpoint allows to get the value of a token.

```http
GET /_/token/value?name=<NAME>
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Query String

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| name | string | The name of the token. | true |

Example:

```http
GET /_/token/value?name=example
```
