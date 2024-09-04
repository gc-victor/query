# Branch

A branch is a copy of a database. The branch command allows to manage the branches of your Query Server, if you are admin.

Usage:

```sh
query branch <COMMAND>
```

It has the following commands:

- `create` - Create a new branch.
- `delete` - Delete a branch.
- `list` - List all the branches.
- `help` - Print this message or the help of the given subcommand(s).

## Create Branch

It will create a new branch.

Usage:

```sh
query branch create
```

It will ask you for the following information:

- Which database would you like to use for creating a branch?
- What is the branch name?

The branches has this format: "**<db_name>.<branch_name>.branch.sql**". For example, if the database name is example.sql and the branch name is dev, the branch will be example.dev.branch.sql. Notice that the extension is removed from the database name to be used as a prefix.

## Delete Branch

It will delete a branch.

Usage:

```sh
query branch delete
```

It will ask you for the following information:

- Which branch database would you like to delete?

## List Branches

It will show you a list of all the branches.

Usage:

```sh
query branch list
```
