# Plugin

Query CLI offers a plugin system that allows you to extend the functionality of the Query Runtime. The plugins are WASM files stored in the `plugins` folder.

Usage:

```sh
query plugin <COMMAND>
```

Commands:

- `install`: Install a plugin from an GitHub repository URL
- `update`: Update plugins
- `push`: Push a plugin or all of them to the server

## Install Plugin

The install command allows you to install a plugin from a GitHub repository URL. The plugin should be a released WASM file. It will download the WASM file and store it in the `plugins` folder and store the plugin information in the `.query/plugins.toml` files.

Usage:

```sh
query plugin install [OPTIONS] <GITHUB_REPO_URL>
```

Example:

```sh
query plugin install https://github.com/gc-victor/query-plugin-argon2
```

Options:

```sh
-e, --exclude <EXCLUDE>  Exclude *.wasm files from the installation
```

## Update Plugin

The update command allows you to update the plugins. It will check the GitHub repository saved in the `.query/plugins.toml` file for new releases and update the plugins.

Usage:

```sh
query plugin update
```

## Push Plugins

The push command allows you to push a plugin or all of them to the server.

Usage:

```sh
query plugin push [PATH]
```

If you set a path, it will push the plugin in the path. If you don't set a path, it will push all the plugins from the `.query/plugins.toml` previously installed.
