# Configuration

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

[task.dev]
bundle = ".query/tasks/bundle.sh"
tailwindcss = "node_modules/.bin/tailwindcss -i ./src/pages/styles.css -o ./dist/styles.css"
```

## Options

- **server** - The settings of the server to deploy
  - **url** - The URL of the server to deploy. It will be requested during the settings process
- **structure** - The structure of the project
  - **migrations_folder** - The folder where the migrations are stored. (Default: src/migrations)
  - **functions_folder** - The folder where the functions are stored. (Default: src/functions)
  - **templates_folder** - The folder where the templates are stored. (Default: templates)
- **esbuild** - The esbuild CLI params configuration for the functions. You can find more information in the [esbuild documentation](https://esbuild.github.io/api/).
- **task** - The task to execute, it is similar to the package.json scripts. You can find more information in the [task documentation](/docs/cli/task.html).

## Environment Variables

The environment variables are located in the **.env** file or global. The variables are the follow:

```yaml
# Server

QUERY_SERVER_PORT=3000 # The port of the server
QUERY_SERVER_APP=true # If it is true, it will start the server as an application
QUERY_SERVER_DBS_PATH=.dbs # The path where the databases are stored
QUERY_SERVER_TOKEN_SECRET=temp_17c7181835bb4de0 # $ openssl rand -hex 32
QUERY_SERVER_ADMIN_EMAIL=admin # The email of the admin user
QUERY_SERVER_ADMIN_PASSWORD=admin # The password of the admin user

# Application

QUERY_APP_ENV=development # The environment of the application
QUERY_APP_QUERY_SERVER=http://localhost:3000 # The URL of the Query Server
QUERY_APP_ALLOWED_ORIGIN=http://localhost:3000 # The allowed origin
```
