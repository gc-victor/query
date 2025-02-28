# Branch

A branch is a copy of a database. The branch endpoint allows to manage the branches of your Query Server.

## POST

The branch endpoint allows to create a new branch.

```http
POST /_/branch
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| db_name | string | The database to use. | true |
| branch_name | string | The name of the branch. | true |

Example:

```json
{
  "db_name": "example.sql",
  "branch_name": "dev"
}
```

The branches has this format: `<db_name>.<branch_name>.branch.sql`. For example, if the database name is `example.sql` and the branch name is `dev`, the branch will be `example.dev.branch.sql`. Notice that the extension is removed from the database name to be used as a prefix.

## GET

The branch endpoint allows to get a list of all the branches.

```http
GET /_/branch
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

To retrieve the list of branches, the system get the list of files in the database directory and filter the files with the extension `.branch.sql`.

## DELETE

The branch endpoint allows to delete a branch.

```http
DELETE /_/branch
```

### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| Authorization | string | The bearer token to connect to the server. | true |

### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| db_name | string | The database to delete. | true |

Example:

```json
{
  "db_name": "example.dev.branch.sql"
}
```

The branches has this format: `<db_name>.<branch_name>.branch.sql`. For example, if the database name is `example.sql` and the branch name is `dev`, the branch will be `example.dev.branch.sql`. Notice that the extension is removed from the database name to be used as a prefix.

Only branches can be deleted, it means files with the extension `.branch.sql`. The primary databases cannot be deleted.