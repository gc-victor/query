# User

The user command allows to manage the users of your Query Server, if you are admin. If you are not admin, you can only change your user password.

Usage:

```sh
query user <SUBCOMMAND>
```

It has the following subcommands:

- `create` - Create a new user.
- `delete` - Delete a user.
- `list` - List all the users.
- `update` - Update a user.
- `password` - Update your user password.
- `help` - Print this message or the help of the given subcommand(s).

## Create User

It will create a new user.

Usage:

```sh
query user create
```

It will ask you for the following information:

- What is her email?
- What is her password?
- Is she an admin user? (Y/n)
- Is she an active user? (Y/n)

## Delete User

It will delete a user.

Usage:

```sh
query user delete
```

It will ask you for the following information:

- What is her email?

## List Users

It will show you a list of all the users.

Usage:

```sh
query user list
```

## Update User

It will update a user.

Usage:

```sh
query user update
```

It will ask you for the following information:

- What is her email?
- What is her new email? (Optional)
- What is her new password? (Optional)
- Is she an admin user? (y/n) (Optional)
- Is she an active user? (y/n) (Optional)
