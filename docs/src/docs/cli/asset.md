# Asset

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
- `-d, --delete` - Delete the asset. It is mandatory to provide the path to the asset
- `-p, --path <PATH>` - Path to the assets
- `-h, --help` - Print help
