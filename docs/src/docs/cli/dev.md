# Dev

Query CLI offers a development mode. It runs the Query Server locally and watches the changes in the files in the `dist`, `src`, and `public` folders. If you change a file, it pushes it to the server.

To use the development mode, it is needed to have installed `query`, `query-server` and `esbuild`, with a global or local installation using `npm` or `pnpm`.

```sh
npm install @qery/query @qery/query-server esbuild
```

Or

```sh
pnpm install @qery/query @qery/query-server esbuild
```

Get more information in the [Install](/docs/install.md) sections.

Also, the minimum configuration in the `Query.toml` file and the `.env` file is required.

The  `Query.toml` file should have the following structure:

```toml
[server]
url = "http://localhost:3000"

[structure]
functions_folder = "src"
```

This is a minimal configuration. You can add more configuration options to the `Query.toml` file. You can find more information in the [Configuration](/docs/configuration.md) section.

The `.env` file should have the following structure:

```yaml
# Server
QUERY_SERVER_PORT=3000
QUERY_SERVER_APP=true
QUERY_SERVER_DBS_PATH=.dbs
QUERY_SERVER_TOKEN_SECRET=1d6005175b5682fb9141515e5336e959 # $ openssl rand -hex 32
QUERY_SERVER_ADMIN_EMAIL=admin
QUERY_SERVER_ADMIN_PASSWORD=admin

# Application
QUERY_APP_ENV=development
QUERY_APP_QUERY_SERVER=http://localhost:3000
QUERY_APP_ALLOWED_ORIGIN=http://localhost:3000
```

Usage:

```sh
query dev
```

Or

```sh
pnpm run dev
```

Or

```sh
npx run dev
```

It uses the `esbuild` to bundle the functions. So, every time you change a function, if there is an error, it will show you the error in the terminal. If there is no error, it will push the function to the server.

Options:

- `-c, --clean` - Clean assets and function databases, and dist folder
- `-v, --verbose` - Show all the logs
- `-h, --help` - Print help

To clean the assets and function databases, and the dist folder, you have to run the following command:

```sh
query dev -c
```

To show all the logs, you have to run the following command:

```sh
query dev -v
```
