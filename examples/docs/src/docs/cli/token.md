# Token

The token command allows to manage the tokens not related to a user of your Query Server, if you are admin.

Usage:

```sh
query token <SUBCOMMAND>
```

It has the following subcommands:

- `create` - Create a new token.
- `delete` - Delete a token.
- `list` - List all the tokens.
- `update` - Update a token.
- `help` - Print this message or the help of the given subcommand(s).

## Create Token

It will create a new token.

Usage:

```sh
query token create
```

It will ask you for the following information:

- What is the name of the token?
- Should have write permissions? (Y/n)
- What is the expiration date in milliseconds? (Optional)

## Delete Token

It will delete a token.

Usage:

```sh
query token delete
```

It will ask you for the following information:

- What is the name of the token?

## List Tokens

It will show you a list of all the tokens.

Usage:

```sh
query token list
```

## Update Token

It will generate a new token maintaining the current name.

Usage:

```sh
query token update
```

It will ask you for the following information:

- What is the name of the token?
- What is the new name of the token? (Optional)
- Should have write permissions? (y/n) (Optional)
- What is the expiration date in milliseconds? (Optional)
