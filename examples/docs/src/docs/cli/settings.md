# Settings

Lets add the server and user settings with the following command:

```sh
query settings
```

It will ask you the following questions:

- Server URL:

You can use a local one for development. By default, it is `http://localhost:3000`.

- Email:

You have to use the same email you used to create the admin user. By default, it is `admin`, only use it for local environments.

- Password:

You have to use the same password you used to create the admin user. By default, it is `admin`, only use it for local environments.

> [!NOTE]
> Once the setting are finished a token will be saved in the `.query/.token` file. This token will be used to authenticate the CLI with the server.
