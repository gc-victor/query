# Install

Query has two main components: the CLI and the Server. The CLI provides a series of commands to manage your databases and deploy your code. The Server is responsible for running your code.

## Using the create command

You can create a new project using the `create` command. Run the following command:

```sh
pnpm dlx @qery/query create
```

OR

```sh
npx @qery/query create
```

It will create a new directory with the project structure and install the necessary dependencies.

Also, you can create a new project from a GitHub repository using the `create` command. Run the following command:

```sh
pnpm dlx @qery/query [GITHUB_REPO_URL]
```

OR

```sh
npx @qery/query [GITHUB_REPO_URL]
```

## Using the package

You can install the CLI using npm or pnpm. Run the following command:

```sh
pnpm install @qery/query @qery/query-server esbuild
```

OR

```sh
npm install @qery/query @qery/query-server esbuild
```

## Using the installer

macOS and Linux (not NixOS, Alpine, or Asahi):

Query CLI:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-installer.sh | sh
```

Query Server:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-server-installer.sh | sh
```

Once it is installed, you can update the CLI and the Server with these commands:

```sh
query-update
query-server-update
```

<div class="markdown-alert">

> [!IMPORTANT]
> ### IMPORTANT!{.markdown-alert-title .font-cal}
> Query uses under the hood [esbuild](https://esbuild.github.io) to
> bundle the functions. So, you have to install esbuild in your project.
> Install it running the following command: `pnpm install esbuild` or `npm install esbuild`.
> Or one of the [other ways to install esbuild](https://esbuild.github.io/getting-started/#other-ways-to-install).

</div>
