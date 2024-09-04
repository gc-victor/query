# Create

Query CLI offers an API that enables users to create a new local project based on predefined templates.

You can choose one of the default templates if you don't use an argument:

```sh
pnpm dlx @qery/query create
```

Or

```sh
npx @qery/query create
```

If you provide a repository URL as an argument, the project will be created using it:

Usage:

```sh
pnpm dlx @qery/query create [REPOSITORY_URL]
```

Or

```sh
npx @qery/query create [REPOSITORY_URL]
```

Example:

```sh
pnpm dlx @qery/query create https://github.com/gc-victor/query-app
```
