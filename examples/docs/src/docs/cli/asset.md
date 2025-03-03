# Asset

Query CLI offers an API that enables users to upload assets to the Query Server. These assets are uploaded to the `query_asset.sql` database and are served in the path `/_/asset/name` or `/_/asset/name_hashed`. The names of the assets are kept in the database as `name` and `name_hashed`. The `name` is the original name of the asset, while the `name_hashed` is a hashed name of the asset, which the hash is based on its content, with the format `dog-000.png`. They have different `Cache-Control` configurations. The `name` has a `Cache-Control` of `public, max-age=300, must-revalidate`, while the `name_hashed` has a `Cache-Control` of `public, max-age=31536000, immutable`.

The assets are stored in memory and purged whenever an asset or function is deployed. You can configure the asset cache store using environment variables. The default values are:

```sh
QUERY_ASSET_CACHE_MAX_CAPACITY = 25 * 1024 * 1024; // 25 MB
QUERY_ASSET_CACHE_TIME_TO_IDLE = 86400;            // 1 day
QUERY_ASSET_CACHE_TIME_TO_LIVE = 2592000;          // 30 days
```

These environment variables must be defined before deploying the Query Server.

Usage:

```sh
query asset [OPTIONS] <PATH>
```

Example:

```sh
query asset ./public
```

Options:

- `-a, --active <ACTIVE>` - Activate status of the asset [default: true]
- `-d, --delete` - Delete the asset. It is mandatory to provide the path to the asset
- `-p, --path <PATH>` - Path to the assets
- `-h, --help` - Print help
