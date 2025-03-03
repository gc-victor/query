# User Token

The user token command allows to manage the user tokens of your Query Server, if you are admin.

Usage:

```sh
query user-token <SUBCOMMAND>
```

It has the following subcommands:

- `create` - Create a new user token.
- `delete` - Delete a user token.
- `list` - List all the user tokens.
- `help` - Print this message or the help of the given subcommand(s).

## Create User Token

It will create a new user token.

Usage:

```sh
query user-token create
```

It will ask you for the following information:

- What is her email?
- Should have write permissions? (Y/n)
- What is the expiration date in milliseconds? (Optional)

## Delete User Token

It will delete a user token.

Usage:

```sh
query user-token delete
```

It will ask you for the following information:

- What is her email?

## List User Tokens

It will show you a list of all the user tokens.

Usage:

```sh
query user-token list
```

## Update User Token

It will generate a new user token maintaining the current email.

Usage:

```sh
query user-token update
```

It will ask you for the following information:

- What is her email?
- Should have write permissions? (y/n) (Optional)
- What is the expiration date in milliseconds? (Optional)
