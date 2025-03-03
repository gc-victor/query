# User Token

The user token endpoint allows to manage the user tokens of the Query Server.

## POST

The user token endpoint allows to create a new user token.

```http
POST /_/user-token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| email | string | The email of the user. | - | true |
| write | boolean | If the token has write permissions. | true | false |
| expiration_date | number | The expiration date in milliseconds. |= updated_at | false |

Example:

```json
{
  "email": "example@example.com",
  "write": true,
  "expiration_date": 1632960000000
}
```

## GET

The user token endpoint allows to get a list of all the user tokens.

```http
GET /_/user-token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

## PUT

The user token endpoint allows to update a user token.

```http
PUT /_/user-token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| email | string | The email of the user. | - | true |
| write | boolean | If the token has write permissions. | false | false |
| expiration_date | number | The expiration date in milliseconds. |= updated_at | false |

Example:

```json
{
  "email": "example@example.com",
  "write": true,
  "expiration_date": 1632960000000
}
```

## DELETE

The user token endpoint allows to delete a user token.

```http
DELETE /_/user-token
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| email | string | The email of the user. | true |

Example:

```json
{
  "email": "example@example.com"
}
```

## GET Value

The user token endpoint allows to get the value of a user token having an access token.

```http
GET /_/user-token/value?email=<EMAIL>
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Query String

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| email | string | The email of the user. | true |

Example:

```http
GET /_/user-token/value?email=example@example.com
```

## POST Value

The user token endpoint allows to create a new user token without having an access token.

```http
POST /_/user-token/value
```

### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| email | string | The email of the user. | true |
| password | string | The password of the user. | true |

Example:

```json
{
  "email": "example@example.com",
  "password": "example"
}
```
