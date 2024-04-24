# ![Query](/docs/query-logo.svg)

Query is a Rust server for your remote SQLite databases with a CLI and API to manage them.

## Table Of Contents

- [Run A Query Server on Fly.io](#run-a-query-server-on-flyio)
  - [Query As An Isolated Service](#query-as-an-isolated-service)
  - [Query Server With Proxy](#query-server-with-proxy)
  - [Query Server App](#query-server-app)
  - [Fly configuration](#fly-configuration)
- [CLI](#cli)
  - [Install](#install)
    - [Use The Installer Scripts](#use-the-installer-scripts)
    - [Download The Binary](#download-the-binary)
  - [Configuration](#configuration)
  - [Commands](#commands)
    - [Settings](#settings)
      - [URL](#url)
      - [Login](#login)
      - [History](#history)
    - [Shell](#shell)
    - [User](#user)
      - [Create User](#create-user)
      - [Delete User](#delete-user)
      - [List Users](#list-users)
      - [Update User Password](#update-user-password)
      - [Update User](#update-user)
    - [User Token](#user-token)
      - [Create User Token](#create-user-token)
      - [Delete User Token](#delete-user-token)
      - [List User Tokens](#list-user-tokens)
      - [Update User Token](#update-user-token)
    - [Token](#token)
      - [Create Token](#create-token)
      - [Delete Token](#delete-token)
      - [List Tokens](#list-tokens)
      - [Update Token](#update-token)
    - [Migration](#migration)
    - [Branch](#branch)
      - [Create Branch](#create-branch)
      - [Delete Branch](#delete-branch)
      - [List Branches](#list-branches)
    - [Generator](#generator)
      - [How it works](#how-it-works)
        - [Database migrations](#database-migrations)
        - [Templates](#templates)
    - [Function](#function)
      - [Handle Request Example](#handle-request-example)
      - [Folder Structure Example](#folder-structure-example)
      - [Query Cache Control](#query-cache-control)
      - [Usage](#usage)
    - [Asset](#asset)
- [APIs](#apis)
  - [Query Endpoint](#query-endpoint)
  - [User Endpoint](#user-endpoint)
  - [User Token Endpoint](#user-token-endpoint)
  - [Token Endpoint](#token-endpoint)
  - [Migration Endpoint](#migration-endpoint)
  - [Branch Endpoint](#branch-endpoint)

## Run A Query Server on Fly.io

We recommend use Query with Fly (<https://fly.io>). It will help you to deploy your server in a few minutes and replicate your databases across the world.

You can use Query as an isolated service or you can use it as a service with a proxy to your App. We will see both options.

### Query As An Isolated Service

Query allows you to set a service with authentication to access remote SQLite databases and possibility to use [Query CLI](https://github.com/gc-victor/query/blob/main/README.md#cli), [Query API](https://github.com/gc-victor/query/blob/main/README.md#apis) and  [Query Studio](https://github.com/gc-victor/query-studio).

### How to use it

Your Dockerfile must include the Query Server. The Dockerfile could be a multistage one, where the last stage should be an `x86_64-unknown-linux-gnu` compatible image. We recommend using a `debian:<suite>-slim` image.

Please refer to the [LiteFS documentation](https://fly.io/docs/litefs/speedrun/) for more information, as it is a crucial system component.

Dockerfile:

```Dockerfile
FROM debian:12-slim

COPY litefs.yml /etc/litefs.yml
COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs

RUN apt-get update -qq && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    sqlite3 \
    fuse3 \
    curl

# Download and installs Query Server
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-server-installer.sh | sh

# It will execute the Query Server and your App
COPY process.sh process.sh
RUN chmod +x process.sh

# Queries databases path
ENV QUERY_SERVER_DBS_PATH="/mnt/dbs"

EXPOSE 3000

CMD ["litefs", "mount"]
```

process.sh:

```bash
#!/bin/bash

/root/.cargo/bin/query-server
```

litefs.yml:

```yml
...
exec:
  - cmd: "./process.sh"
...
```

### Query Server With Proxy

Query allows you to set a proxy to an App in the same VM. It provides you access to the databases directly from your application while enjoying the benefits of using Query, such as [Query CLI](https://github.com/gc-victor/query/blob/main/README.md#cli), [Query API](https://github.com/gc-victor/query/blob/main/README.md#apis) and  [Query Studio](https://github.com/gc-victor/query-studio).

### How to use it

In your Dockerfile, you must include the Query Server and your Application together. The Dockerfile could be a multistage one, where the last stage should be an `x86_64-unknown-linux-gnu` compatible image. We recommend using a `debian:<suite>-slim` image.

For this example, we will use Bun as our App. You can use any other language or framework.

Please refer to the [LiteFS documentation](https://fly.io/docs/litefs/speedrun/) for more information, as it is a crucial system component.

Dockerfile:

```Dockerfile
FROM debian:12-slim AS runtime

COPY litefs.yml /etc/litefs.yml
COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs

RUN apt-get update -qq && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    sqlite3 \
    fuse3 \
    curl

# Download and installs Query Server
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-server-installer.sh | sh

# It will execute the Query Server and your App
COPY process.sh process.sh
RUN chmod +x process.sh

# Queries databases path
ENV QUERY_SERVER_DBS_PATH="/mnt/dbs"
# Enable Query Server Proxy
ENV QUERY_SERVER_PROXY="true"
# Your App port
ENV QUERY_SERVER_PROXY_PORT="3001"

# DO WHATEVER YOU NEED TO INSTALL YOUR APP

EXPOSE 3000

CMD ["litefs", "mount"]
```

For multi-process applications, you can use the "Just use Bash", as we do in this example, to start the Query Server and your App. [Fly proposes](https://fly.io/docs/app-guides/multiple-processes/) different ways to manage multiple processes, so please use the one you feel more comfortable with.

process.sh:

```bash
#!/bin/bash

set -m
/root/.cargo/bin/query-server &
__START_YOUR_APP__ &
fg %1
```

Please, change `__START_YOUR_APP__` with the command to start your App.

litefs.yml:

```yml
...
exec:
  - cmd: "./process.sh"
...
```

Please, visit the example/proxy folder to see a working example. You will have to rename the `fly.toml.dist` to `fly.toml` to be able to deploy it and follow the steps from [Run a Query Server](https://github.com/gc-victor/query?tab=readme-ov-file#run-a-query-server) to finalize the process.

### Query Server App

Removing the `/_/function` prefix using an environment variable is possible. This can be useful when using the Query Server to serve Pages and APIs. You can set this configuration using the following environment variable:

```sh
QUERY_SERVER_APP=true
```

You can now access pages using `/rest_of_the_path` instead of `/_/function/pages/rest_of_the_path`. Similarly, APIs will be `/api/rest_of_the_path` instead of `/_/function/api/rest_of_the_path`. As usual, every function will be served using the `/_/function` prefix.

It is important to note that the `QUERY_SERVER_APP` environment variable is optional. The`/_/function` path will be used for every case if you don't provide it.

[More information about the Function feature](#function).

### Fly configuration

If it is the first time using Fly, you can follow the [Hands-on with Fly.io](https://fly.io/docs/hands-on/) guide to install the CLI, sign up and sign in.

Once you have the Fly CLI installed, you have to rename the `fly.toml.dist` to `fly.toml`, and update it with your app name and the primary region running the following command:

```sh
fly launch
```

It is time to set the environment variables for your app. You can do it running the following commands:

Token secret:

```sh
fly secrets set QUERY_SERVER_TOKEN_SECRET=$(openssl rand -hex 32)
```

> **Note**: If you don't have openssl installed, you can also use
> [1Password](https://1password.com/password-generator) to generate a random
> secret, just replace `$(openssl rand -hex 32)` with the generated secret.

Admin email:

```sh
fly secrets set QUERY_SERVER_ADMIN_EMAIL=USE_YOUR_EMAIL
```

Admin password:

```sh
fly secrets set QUERY_SERVER_ADMIN_PASSWORD=USE_A_SECURE_PASSWORD
```

We use LiteFS, a Fly addon that provides a simple way to replicate your SQLite databases in the cloud. To use LiteFS you need to configure consul. You can do it running the following commands:

```sh
fly consul attach
```

For the backups of your databases you have to create a LiteFS Cloud cluster in [the LiteFS section](https://fly.io/dashboard/personal/litefs) of the fly.io dashboard. Take note of your auth token (you’ll need it later). LiteFS Cloud is optional, but highly recommended if your data is important to you!

```sh
fly secrets set LITEFS_CLOUD_TOKEN=YOUR_LITEFS_CLOUD_AUTH_TOKEN
```

Then you can deploy your app running:

```sh
fly deploy
```

Your app is currently running on a single machine. To ensure high availability, especially for production apps, Fly strongly recommend running at least 2 instances. You can scale up the number of machines using the `fly machine clone` command in the CLI. Please, have in mind that you can add that machine to an other region.

```sh
fly m clone
```

Or

```sh
fly m clone --select --region A_REGION
```

*Example: `fly m clone --select --region lhr`* (London)

To get a list of rigions you can run the following command:

```sh
fly platform regions
```

## CLI

### Install

#### Use The Installer Scripts

npm package:

```sh
npm install -g @qery/query
```

OR

```sh
pnpm install -g @qery/query
```

macOS and Linux (not NixOS, Alpine, or Asahi):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-installer.sh | sh
```

Windows PowerShell:

```powershell
irm https://github.com/gc-victor/query/releases/latest/download/query-installer.ps1 | iex
```

#### Download The Binary

<https://github.com/gc-victor/query/releases/latest>

### Dev

To run the development mode, you have to run the following command:

```sh
query dev
```

It will run the query-server and watch the changes in the files in the `dist`, `src` and `public` folders. If you change a file, it will push them to server.

### Configuration

The configuration file is located in the **/.query** folder. It is a TOML file named **Query\.toml**. The file format is as follows:

```toml
[server]
url = "http://localhost:3000"

[structure]
functions_folder = "src"
migrations_folder = "migrations"

[esbuild]
"loader:.html" = "text"
"loader:.svg" = "text"
```

#### Options

- **server** - The settings of the server to deploy
  - **url** - The URL of the server to deploy. It will be requested during the settings process
- **structure** - The structure of the project
  - **migrations_folder** - The folder where the migrations are stored. (Default: src/migrations)
  - **functions_folder** - The folder where the functions are stored. (Default: src/functions)
  - **templates_folder** - The folder where the templates are stored. (Default: templates)
- **esbuild** - The esbuild CLI params configuration for the functions. You can find more information in the [esbuild documentation](https://esbuild.github.io/api/).



### Commands

Following we will see the commands you can use with the CLI.

#### Settings

Lets start by adding the settings of your server.

```sh
query settings
```

It will ask you the following questions:

##### URL

- What is the server URL?

You can use a local one for development, or if you want to use Fly for your development deploys or access your remote databases, you can run the following command to get your Fly URL:

```sh
fly status
```

You will have to use the Hostname plus as a prefix the protocol `https://`.

*Example: `https://query-server.fly.dev`*

Where `query-server.fly.dev` is the Hostname.

##### Login

You need to log in to get the token to connect to your Query Server. The token will be saved in the `.query/.token` file.

- What is her email?

You have to use the same email you used to create the admin user.

- What is her password?

You have to use the same password you used to create the admin user.

##### History

- Do you want to save the history of your shell? (Y/n)

If you choose `Y` the history will be saved in the `.query/.history` file.

#### Shell

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

#### User

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

##### Create User

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

##### Delete User

It will delete a user.

Usage:

```sh
query user delete
```

It will ask you for the following information:

- What is her email?

##### List Users

It will show you a list of all the users.

Usage:

```sh
query user list
```

##### Update User Password

It will update your user password.

Usage:

```sh
query user password
```

It will ask you for the following information:

- What is your new password?

##### Update User

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

### User Token

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

#### Create User Token

It will create a new user token.

Usage:

```sh
query user-token create
```

It will ask you for the following information:

- What is her email?
- Should have write permissions? (Y/n)
- What is the expiration date in milliseconds? (Optional)

#### Delete User Token

It will delete a user token.

Usage:

```sh
query user-token delete
```

It will ask you for the following information:

- What is her email?

#### List User Tokens

It will show you a list of all the user tokens.

Usage:

```sh
query user-token list
```

#### Update User Token

It will generate a new user token maintaining the current email.

Usage:

```sh
query user-token update
```

It will ask you for the following information:

- What is her email?
- Should have write permissions? (y/n) (Optional)
- What is the expiration date in milliseconds? (Optional)

### Token

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

#### Create Token

It will create a new token.

Usage:

```sh
query token create
```

It will ask you for the following information:

- What is the name of the token?
- Should have write permissions? (Y/n)
- What is the expiration date in milliseconds? (Optional)

#### Delete Token

It will delete a token.

Usage:

```sh
query token delete
```

It will ask you for the following information:

- What is the name of the token?

#### List Tokens

It will show you a list of all the tokens.

Usage:

```sh
query token list
```

#### Update Token

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

### Migration

The migration command allows to manage the migrations of your Query Server, if you are admin.

Migration file:

- The migration file should be in the format of &lt;version&gt;_&lt;name&gt;_&lt;type&gt;.sql
- The version should be in the format of YYYYMMDD
- The name should be in the format of &lt;name&gt;_&lt;description&gt;
- The type should be up or down

Usage:  

```sh
query migration <DB_NAME> <PATH>
```

### Branch

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

#### Create Branch

It will create a new branch.

Usage:

```sh
query branch create
```

It will ask you for the following information:

- Which database would you like to use for creating a branch?
- What is the branch name?

The branches has this format: "**<db_name>.<branch_name>.branch.sql**". For example, if the database name is example.sql and the branch name is dev, the branch will be example.dev.branch.sql. Notice that the extension is removed from the database name to be used as a prefix.

#### Delete Branch

It will delete a branch.

Usage:

```sh
query branch delete
```

It will ask you for the following information:

- Which branch database would you like to delete?

#### List Branches

It will show you a list of all the branches.

Usage:

```sh
query branch list
```

### Generator

Query's generator is a tool that helps you create a set of files using a simple command that represents a table's structure. It lets you quickly and easily create the needed files without writing everything from scratch.

Example:

```sh
query generator blog.sql post title:string content:text
```

Format:

```sh
query generator <DATABA> <TABLE> <COLUMNS[COLUMN:TYPE]>
```

#### Column Types

The following table illustrates the mapping between Column Types, TypeScript, and SQLite data types:

| ColumnType | TypeScript | SQLite  |
|------------|------------|---------|
| blob       | Blob       | BLOB    |
| boolean    | boolean    | BOOLEAN |
| number     | number     | INTEGER |
| integer    | number     | INTEGER |
| float      | number     | REAL    |
| real       | number     | REAL    |
| timestamp  | string     | INTEGER DEFAULT (strftime('%s', 'now')) |
| string     | string     | TEXT    |
| text       | string     | TEXT    |
| uuid       | string     | TEXT UNIQUE CHECK ({column_name} != '') DEFAULT (uuid()) |

#### How it works

The generator does two things:

- Generate the database migrations files to update your database
- Generate a set of files based on templates

##### Database migrations

The migration generated will use the command to create the table and the columns. The migration will be stored in the **/migrations** folder inside a folder with the database name (Ex. blog.sql). It will generate two files with the format of **`<version>_<name>_<type>.sql`**. The version will have the format of YYYYMMDDHHMMSS, the name should be in the format of `<name>_<description>`, and the types will be **up** and **down**.

You can find more information about migrations in the [Migration](#migration) section.

##### Templates

The templates used to generate files are stored in the **/templates** folder or a custom folder specified in the Query's config file.

```toml
[structure]
templates_folder = other-template-folder
```

Query uses a basic template system that we will describe in detail below.

There are some dynamic variables based on the command params that you can use to generate the file content:

- **{{ database }}**: The database where the migration will be executed
- **{{ table }}**<sup>1</sup>: The name of the table
- **{{ columnsLength }}**: The number of the columns
- **{{ columns }}**: The list of columns specified
  - **{{ columnIndex }}**: The index value in the loop
  - **{{ columnFirst }}**: The first column in the loop
  - **{{ columnLast }}**: The last column in the loop
  - **{{ columnName }}**<sup>2</sup> <sup>1</sup>: The name of the column
  - **{{ columnTypeMatchTS }}**: The match of the type of the column with the TypeScript type
  - **{{ columnsListOfUniqueTSTypes }}**: A list of the matches between column type and TypeScript type in lowercase
  - **{{ columnType }}**<sup>2</sup> <sup>1</sup>: The type of the column

<sub>1 The table, the columnName, and the columnType have name variants you can use in your templates.</sub>

<sub>2 To get the columnName and columnType, it is required to iterate over the columns.</sub>

As we have commented, you can use some name variants in your templates for the table, columnName, and columnType. The name variants are based on the command that you will use to generate the files.

**Variants:**

- **camelCase** (Ex. testName)
- **hyphenCase** (Ex. test-name)
- **snakeCase** (Ex. test_name)
- **dotCase** (Ex. test.name)
- **pathCase** (Ex. test/name)
- **constantCase** (Ex. TEST_NAME)
- **pascalCase** (Ex. TestName)
- **capitalCase** (Ex. Test Name)
- **lowerCase** (Ex. test name)
- **sentenceCase** (Ex. Test name)
- **upperCase** (Ex. TEST NAME)
- **upperCaseFirst** (Ex. Test name)
- **lowerCaseFirst** (Ex. test name)

**Variables:**

```tmpl
{{ tableCamelCase }}
{{ tableHyphenCase }}
{{ tableSnakeCase }}
{{ tableDotCase }}
{{ tablePathCase }}
{{ tableConstantCase }}
{{ tablePascalCase }}
{{ tableCapitalCase }}
{{ tableLowerCase }}
{{ tableSentenceCase }}
{{ tableUpperCase }}
{{ tableUpperCaseFirst }}
{{ tableLowerCaseFirst }}
{{ columnNameCamelCase }}
{{ columnNameHyphenCase }}
{{ columnNameSnakeCase }}
{{ columnNameDotCase }}
{{ columnNamePathCase }}
{{ columnNameConstantCase }}
{{ columnNamePascalCase }}
{{ columnNameCapitalCase }}
{{ columnNameLowerCase }}
{{ columnNameSentenceCase }}
{{ columnNameUpperCase }}
{{ columnNameUpperCaseFirst }}
{{ columnNameLowerCaseFirst }}
{{ columnTypeCamelCase }}
{{ columnTypeHyphenCase }}
{{ columnTypeSnakeCase }}
{{ columnTypeDotCase }}
{{ columnTypePathCase }}
{{ columnTypeConstantCase }}
{{ columnTypePascalCase }}
{{ columnTypeCapitalCase }}
{{ columnTypeLowerCase }}
{{ columnTypeSentenceCase }}
{{ columnTypeUpperCase }}
{{ columnTypeUpperCaseFirst }}
{{ columnTypeLowerCaseFirst }}
```

The template system provides two operations to use in your templates:

**If:**

```html
{% if table == "post" %}
  <p>This is a Post.</p>
{% else %}
  <p>This isn't a Post.</p>
{% endif %}
```

**For:**

```html
{% for column in columns %}
  <p>{% column.columnName %}</p>
{% endfor %}
```

With the previous information, you can create a set of files based on the table's schema. These files should be placed in the templates folder, with the folder structure used to generate files in their respective locations. The templates folder structure should match that of the *functions_folder*, which is typically configured as */src*, although you will need to configure it yourself. You can find more information about the configuration process in the [Configuration](#configuration) section.

Example from the [query-app](https://github.com/gc-victor/query-app) project:

API:

```sh
templates
├── api
│   ├── admin
│   │   ├── login
│   │   │   └── **.index.ts
│   │   └── **
│   │       ├── delete.index.ts
│   │       ├── get.index.ts
│   │       ├── post.index.ts
│   │       ├── put.index.ts
│   │       └── uuid
│   │           └── get.[slug].ts
│   └── **
│       ├── delete.index.ts
│       ├── get.index.ts
│       ├── post.index.ts
│       ├── put.index.ts
│       └── uuid
│           └── get.[slug].ts
└── ...
```

Pages:

```sh
templates
├── pages
│   ├── admin
│   │   ├── components
│   │   │   └── ...
│   │   ├── get.index.ts
│   │   ├── login
│   │   │   └── ...
│   │   ├── **
│   │   │   ├── get.index.tsx
│   │   │   ├── island
│   │   │   │   └── **.island.ts
│   │   │   ├── **.form.view.tsx
│   │   │   └── **.view.tsx
│   │   └── utils
│   │       └── ..
│   ├── components
│   │   └── ..
│   ├── get.index.tsx
│   ├── layouts
│   │   └── ...
│   ├── **
│   │   ├── excerpt.tsx
│   │   ├── get.index.tsx
│   │   └── [slug]
│   │       ├── get.index.tsx
│   │       └── **.tsx
│   └── styles.css
└── ...
```

Notice that the **"\*\*"** is a placeholder for that will be replaced by the table name of the command.

### Function

A function is a JavaScript function that is executed in the Query Server and it has access to the databases.

The function should be in the format of:

```js
export async function handleRequest(req) {
    return new Response("This is the body!", {
      status: 200,
      headers: {
          "content-type": "text/plain",
      },
  });
}
```

The function has to export a function called `handleRequest` that receives a [Request](https://developer.mozilla.org/en-US/docs/Web/API/Request) and returns a [Response](https://developer.mozilla.org/en-US/docs/Web/API/Response).

To use a database you have to create a connection to the database:

```js
const db = new Database("example.sql");
```

The `Database` constructor receives the name of the database. If the database is found, it will create a connection to the database. It will provide the following methods:

- `query` - To read data from the database.
- `execute` - To write data in the database.

A `query` and an `execute` can have params. The params are bound to the parameters based on the order of the array or an object with the format of `:AAA`, `$AAA`, or `@AAA` that serve as placeholders for values that are bound to the parameters at a later time. The params are optional.

As Query uses [LiteFS proxy](https://fly.io/docs/litefs/config/#http-proxy-server), you have to remember to use `GET` to read data and `DELETE|POST|PUT|PATCH` to write data.

#### Handle Request Example

```js
// get.index.js
export async function handleRequest(req) {
    const db = new Database("example.sql");

    const result = await db.query("SELECT * FROM example WHERE id = ?", [1]);

    return new Response(JSON.stringify({data: result}), {
      status: 200,
      headers: {
          "content-type": "application/json",
      },
  });
}
```

Query CLI provides an API to resolving routes against file-system paths and using the file names. To use functions it is required to follow the next structure:

#### Folder Structure Example

```sh
functions
├── get.index.js // GET "/"
├── post.index.js // POST "/"
├── example
    ├── get.index.js // GET "/example"
    └── get.[slug].js // GET "/example/:slug"
├── [slug]
    └── get.index.js  // GET "/:slug"
...
```

By default the folder to contain the functions has to be called `functions`. You can use another one by pointing to it, but we will explain it with more detail below.

It is important to note that the method used in a file is determined by the prefix `(delete|get|patch|post|put).*`, while the remaining part of the file name defines the final segment of the route. For instance, if the file name ends with `index`, it will be the root of the route, and if it is `[slug]`, it will be a route with a slug. The slug is a placeholder for a value used in the route.

To define the different segments of the route, you must use the folder structure. For example, if you want to use the path `/example/:slug`, you have to create a folder called `example` and inside it a file called `get.[slug].js`. If you want to use the route `/:slug`, you have to create a folder called `[slug]` and inside of it a file called `get.index.js`. If you want to use the route `/`, you must create a file called `get.index.js`.

#### Query Cache Control

The Query Server has a feature that helps avoid compiling functions that have not been modified, which in turn speeds up each response. This feature is managed using the `Query-Cache-Control` header and specifying the `max-age`, in milliseconds, in the header response of the `handleRequest` function. The function response is stored in the `cache_function` table of the `query_cache_function.sql` database. If needed, the cache can be purged by either deleting the row related to a path or by deleting the entire cache from the `cache_function` table.

```js
// get.index.js
export async function handleRequest(req) {
    const db = new Database("example.sql");

    const result = await db.query("SELECT * FROM example WHERE id = ?", [1]);

    return new Response(JSON.stringify({data: result}), {
      status: 200,
      headers: {
          "Content-Type": "application/json",
          "Query-Cache-Control": "max-age=3600000", // 1 hour
      },
  });
}
```

#### Usage

Query uses under the hood [esbuild](https://esbuild.github.io) to bundle the functions. So, first you have to install esbuild globally:

```sh
npm install --global esbuild
```

Or

```sh
pnpm install --global esbuild
```

To use the functions you have to run the following command:

```sh
query function <PATH>
```

The path is optional. If you don't provide it, it will use the default path `functions`. You can use the path to point to another folder or a function file.

#### Example

```sh
query function
```

It will deploy all the functions to the Query Server. A simple cache is implemented to avoid deploying functions that have not changed.

```sh
query function another-functions-folder
```

It will deploy all the functions in the `another-functions-folder` folder to the Query Server.

```sh
query function functions/get.index.js
```

It will deploy the `get.index.js` function to the Query Server.

```sh
query function functions/get.index.js --delete
```

It will delete the `get.index.js` function from the Query Server.

### Asset

Query CLI offers an API that enables users to upload assets to the Query Server. These assets are uploaded to the `query_asset.sql` database and are served in the path `/_/asset/name` or `/_/asset/name_hashed`. The names of the assets are kept in the database as `name` and `name_hashed`. The `name` is the original name of the asset, while the `name_hashed` is a hashed name of the asset, which the hash is based on its content, with the format `dog-000.png`. They have different `Cache-Control` configurations. The `name` has a `Cache-Control` of `public, max-age=300, must-revalidate`, while the `name_hashed` has a `Cache-Control` of `public, max-age=31536000, immutable`.

Usage:

```sh
query asset [OPTIONS] <PATH>
```

Example:

```sh
query asset ./assets
```

Options:

- `-a, --active <ACTIVE>` - Activate status of the asset [default: true]  
- `-d, --delete` - Delete the asset It is mandatory to provide the path to the asset
- `-p, --path <PATH>` - Path to the assets
- `-h, --help` - Print help

## APIs

Following we will see the API endpoints you can use with the Query Server.

### Query Endpoint

The `query` endpoint allows to execute queries in the databases. Using the `GET` method, the query is executed in the database closest to the user's region, thanks to the LiteFS proxy. Using the `POST` method, the query is executed in the primary database.

#### POST

The `query` endpoint allows to execute a query in the primary database.

```http
POST /_/query
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `db_name` | `string` | The database to use. | true |
| `query` | `string` | The query to execute. | true |
| `params` | `object` \| `array` | The params to use in the query. | false |

The params object should use kyes with the format ":AAA", "$AAA", or "@AAA" that serve as placeholders for values that are bound to the parameters at a later time.

Example:

```json
{
  "db_name": "example.sql",
  "query": "SELECT * FROM example WHERE id = :id",
  "params": {
    ":id": 1
  }
}
```

In the case of the array, the values are bound to the parameters based on the order of the array.

Example:

```json
{
  "db_name": "example.sql",
  "query": "SELECT * FROM example WHERE id = ?",
  "params": [1]
}
```

#### GET

By using the `GET` method, data can be retrieved with less latency from the database closest to the user's region, thanks to the LiteFS proxy.

```http
GET /_/query?db_name=<DB_NAME>&query=<QUERY>&params=<PARAMS>
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Query String

| Name | Type | Format | Description | Required |
| :--- | :--- | :--- | :--- | :--- |
| `db_name` | `string` | - | The database to use. | true |
| `query` | `string` | URL Encoded | The `SELECT` query. | true |
| `params` | `object` \| `array` | URL Encoded | The params to use in the query. | false |

Example:

```http
GET /_/query?db_name=example.sql&query=SELECT%20*%20FROM%20example%20WHERE%20id%20%3D%20%3F&params=%5B1%5D
```

### User Endpoint

The user endpoint allows to manage the users of the Query Server.

#### POST

The `user` endpoint allows to create a new user.

```http
POST /_/user
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | - | true |
| `password` | `string` | The password of the user. | - | true |
| `admin` | `boolean` | If the user is admin. | false | false |
| `active` | `boolean` | If the user is active. | true | false |

Example:

```json
{
  "email": "example@example.com",
  "password": "example",
  "admin": false,
  "active": true
}
```

#### PUT

The `user` endpoint allows to update a user.

```http
PUT /_/user
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | - | true |
| `new_email` | `string` | The new email of the user. | - | false |
| `new_password` | `string` | The new password of the user. | - | false |
| `admin` | `boolean` | If the user is admin. | false | false |
| `active` | `boolean` | If the user is active. | true | false |

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

#### PUT Password

The `user/password` endpoint allows to update the password of a user. The user is inferred from the token.

```http
PUT /_/user/password
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `password` | `string` | The password of the user. | true |

Example:

```json
{
  "password": "example"
}
```

#### DELETE

The user endpoint allows to delete a user.

```http
DELETE /_/user
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | true |

Example:

```json
{
  "email": "example@example.com"
}
```

### User Token Endpoint

The user token endpoint allows to manage the user tokens of the Query Server.

#### POST

The user token endpoint allows to create a new user token.

```http
POST /_/user-token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | - | true |
| `write` | `boolean` | If the token has write permissions. | true | false |
| `expiration_date` | `number` | The expiration date in milliseconds. |= updated_at | false |

Example:

```json
{
  "email": "example@example.com",
  "write": true,
  "expiration_date": 1632960000000
}
```

#### GET

The user token endpoint allows to get a list of all the user tokens.

```http
GET /_/user-token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

#### PUT

The user token endpoint allows to update a user token.

```http
PUT /_/user-token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | - | true |
| `write` | `boolean` | If the token has write permissions. | false | false |
| `expiration_date` | `number` | The expiration date in milliseconds. |= updated_at | false |

Example:

```json
{
  "email": "example@example.com",
  "write": true,
  "expiration_date": 1632960000000
}
```

#### DELETE

The user token endpoint allows to delete a user token.

```http
DELETE /_/user-token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | true |

Example:

```json
{
  "email": "example@example.com"
}
```

#### GET Value

The user token endpoint allows to get the value of a user token having an access token.

```http
GET /_/user-token/value?email=<EMAIL>
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Query String

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | true |

Example:

```http
GET /_/user-token/value?email=example@example.com
```

#### POST Value

The user token endpoint allows to create a new user token without having an access token.

```http
POST /_/user-token/value
```

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `email` | `string` | The email of the user. | true |
| `password` | `string` | The password of the user. | true |

Example:

```json
{
  "email": "example@example.com",
  "password": "example"
}
```

### Token Endpoint

The token endpoint allows to manage the tokens not related to a user.

#### POST

The token endpoint allows to create a new token.

```http
POST /_/token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `name` | `string` | The name of the token. | - | true |
| `expiration_date` | `number` | The expiration date in milliseconds. |= updated_at | false |
| `active` | `boolean` | If the token is active | true | false |
| `write` | `boolean` | If the token has write permissions. | true | false |

Example:

```json
{
  "name": "example",
  "expiration_date": 1632960000000,
  "active": true,
  "write": true
}
```

#### GET

The token endpoint allows to get a list of all the tokens.

```http
GET /_/token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

#### PUT

The token endpoint allows to update a token.

```http
PUT /_/token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `name` | `string` | The name of the token. | - | true |
| `expiration_date` | `number` | The expiration date in milliseconds. |= updated_at | false |
| `active` | `boolean` | If the token is active | true | false |
| `write` | `boolean` | If the token has write permissions. | true | false |

Example:

```json
{
  "name": "example",
  "expiration_date": 1632960000000,
  "active": true,
  "write": true
}
```

#### DELETE

The token endpoint allows to delete a token.

```http
DELETE /_/token
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `name` | `string` | The name of the token. | true |

#### GET Value

The token endpoint allows to get the value of a token.

```http
GET /_/token/value?name=<NAME>
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Query String

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `name` | `string` | The name of the token. | true |

Example:

```http
GET /_/token/value?name=example
```

### Migration Endpoint

The migration endpoint allows to manage the migrations of the Query Server.

#### POST

The migration endpoint allows to execute a migration in the primary database.

```http
POST /_/migration
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `db_name` | `string` | The database to use. | true |
| `query` | `string` | The query to execute. | true |

Example:

```json
{
  "db_name": "example.sql",
  "query": "CREATE TABLE example (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
}
```

### Branch Endpoint

A branch is a copy of a database. The branch endpoint allows to manage the branches of your Query Server, if you are admin.

#### POST

The branch endpoint allows to create a new branch.

```http
POST /_/branch
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `db_name` | `string` | The database to use. | true |
| `branch_name` | `string` | The name of the branch. | true |

Example:

```json
{
  "db_name": "example.sql",
  "branch_name": "dev"
}
```

The branches has this format: `<db_name>.<branch_name>.branch.sql`. For example, if the database name is `example.sql` and the branch name is `dev`, the branch will be `example.dev.branch.sql`. Notice that the extension is removed from the database name to be used as a prefix.

#### GET

The branch endpoint allows to get a list of all the branches.

```http
GET /_/branch
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

To retrieve the list of branches, the system get the list of files in the database directory and filter the files with the extension `.branch.sql`.

#### DELETE

The branch endpoint allows to delete a branch.

```http
DELETE /_/branch
```

##### Headers

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `Authorization` | `string` | The bearer token to connect to the server. | true |

##### Body

| Name | Type | Description | Required |
| :--- | :--- | :--- | :--- |
| `db_name` | `string` | The database to delete. | true |

Example:

```json
{
  "db_name": "example.dev.branch.sql"
}
```

The branches has this format: `<db_name>.<branch_name>.branch.sql`. For example, if the database name is `example.sql` and the branch name is `dev`, the branch will be `example.dev.branch.sql`. Notice that the extension is removed from the database name to be used as a prefix.

Only branches can be deleted, it means files with the extension `.branch.sql`. The primary databases cannot be deleted.
