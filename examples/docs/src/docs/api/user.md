# User

The user endpoint allows to manage the users of the Query Server.

## POST

The `user` endpoint allows to create a new user.

```http
POST /_/user
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| email | string | The email of the user. | - | true |
| password | string | The password of the user. | - | true |
| admin | boolean | If the user is admin. | false | false |
| active | boolean | If the user is active. | true | false |

Example:

```json
{
  "email": "example@example.com",
  "password": "example",
  "admin": false,
  "active": true
}
```

## PUT

The `user` endpoint allows to update a user.

```http
PUT /_/user
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| email | string | The email of the user. | - | true |
| new_email | string | The new email of the user. | - | false |
| new_password | string | The new password of the user. | - | false |
| admin | boolean | If the user is admin. | false | false |
| active | boolean | If the user is active. | true | false |

Example:

```json
{
  "email": "example@example.com",
  "new_email": "new-example@example.com",
  "new_password": "example",
  "admin": false,
  "active": true
}
```

## DELETE

The user endpoint allows to delete a user.

```http
DELETE /_/user
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
