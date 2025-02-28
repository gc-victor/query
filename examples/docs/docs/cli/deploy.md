# Deploy

The Deploy command in Query CLI allows you to easily deploy your Query project to a server.

## Usage

```sh
query deploy [OPTIONS]
```

## Options

- `--env`: Use environment variables for deployment configuration. If not specified, the CLI will prompt for required information.
- `--no-cache`: Clear the deployment cache. If not specified, the cache will be preserved before deployment.

## Environment Variables

When using the `--env` option or for non-interactive deployments, the following environment variables can be set:

- `QUERY_DEPLOY_URL`: The URL of the Query server to deploy to.
- `QUERY_DEPLOY_TOKEN`: The authentication token for deployment.
- `QUERY_DEPLOY_EMAIL`: The email address for authentication (if token is not provided).
- `QUERY_DEPLOY_PASSWORD`: The password for authentication (if token is not provided).

## Interactive Deployment

If environment variables are not set and the `--env` option is not used, the CLI will prompt for the following information:

1. **Server URL**: The URL of the Query server to deploy to. Defaults to `http://localhost:3000` or the value of `QUERY_SERVER_PORT` environment variable.
2. **Email**: The email address for authentication. Defaults to the value of `QUERY_SERVER_ADMIN_EMAIL` environment variable.
3. **Password**: The password for authentication.

## Deployment Process

1. The CLI first checks for required environment variables or prompts for missing information.
2. It then retrieves a user token for authentication if not provided.
3. If the `--no-cache` option is used, the deployment cache is cleared by removing the `.query/.cache` directory.
4. The `query task deploy` command is executed with the necessary environment variables set.

## Cache Handling

By default, the deployment process preserves the cache before deploying. If you want to clear the cache (e.g., for clean subsequent deployments), use the `--no-cache` option:

```sh
query deploy --no-cache
```

This will clear the cache before deployment, ensuring a fresh start.

## Error Handling

- If required environment variables are missing when using the `--env` option, the CLI will display an error message and exit.
- If there are issues retrieving the user token or during the deployment process, appropriate error messages will be displayed.

## Output

- The CLI provides visual feedback during the deployment process using colored output.
- A success message is displayed upon completion of the deployment.

## Notes

- The deployment process uses the `query task deploy` command, which should be defined in your Query project's task configuration.
- Make sure your Query project is properly set up and configured before running the deploy command.
- Consider using the `--no-cache` option for clean deployments when you know the cache is invalid (e.g., for a change of context from a local environment to a remote one).
