# Shell

The shell command opens a SQLite shell to manage the remote database locally.

Usage:

```sh
query shell <DB_NAME>
```

It has the following commands:

- `.quit` - Exit the shell.
- `.tables [?PATTERN?]` - List names of tables matching a LIKE pattern.
- `.schema [?TABLE?]` - Show the CREATE statements. If TABLE specified, only show tables matching LIKE pattern TABLE.

It saves the command history in the `.query/.history` file.
