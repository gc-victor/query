# ![Query](/docs/images/query-logo.svg)

## Introduction

Query is a one-file solution that simplifies web development by providing a range of features designed to simplify the web stack of your projects for running code, managing assets, caching, and handling databases. Its goal is to make website development and deployment more accessible and efficient by eliminating the need for multiple components and services:

1. **Database Server**: You don't need to use a database server like PostgreSQL or MySQL. Instead, you can connect to SQLite databases remotely via APIs, command-line interface (CLI), and Query Functions.

1. **JavaScript Runtime**: There's no need for a JavaScript runtime environment such as NodeJS, Deno, or Bun. Query offers its own runtime for executing JavaScript.

1. **Backend Framework**: You don't need backend frameworks like NextJS, Remix, or Hono because Query provides its own routing system and functions to serve web pages.

1. **Caching System**: There's no need for an external caching system like Redis. Query caches functions directly in SQLite and serves them super fast.

1. **Storage System**: There's no requirement for an external storage system like Amazon S3 for storing assets, as Query stores and serves them from SQLite.

1. **Scaffolding Tool**: You don't need additional tools to generate starter code because Query includes its own scaffolding capabilities.

1. **Back Office Admin Area**: The need to develop an administrative interface is removed because Query includes a generator for this purpose.
## Quick Start

To create a new project, run the following command:

```sh
pnpm dlx @qery/query create 
```

Or

```sh
npx @qery/query create
```

You will have three options:

- **application**: A project with a basic structure to create a web application.
- **counter**: A basic project with a counter function.
- **minimal**: A minimal project with a single server function.

Choose the one that best fits your needs and follow the instructions. It will create a new directory with the project structure and install the necessary dependencies.

After creating the project, the command will print the following steps to run the project.

```sh
cd <PROJECT_NAME>
```

AND

```sh
pnpm query dev
```

OR

```sh
npx query dev
```

That's it. In less than a minute, you will be able to see the project running at <http://localhost:3000>.

## Links

- Homepage: <https://qery.io>
- Docs: <https://qery.io/docs>
- Discord: <https://discord.gg/pXmHqTXv>

## License

Query is licensed under the [MIT License](LICENSE).

## Change Log

See [CHANGELOG.md](CHANGELOG.md).

