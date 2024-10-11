# Server on Fly.io

We recommend using Query with [Fly.io](https://fly.io). It will help you deploy your server in a few minutes and replicate your databases worldwide.

You can use the Query Server as an isolated service, as it provides authentication to access remote SQLite databases, as a service with a proxy to your application, or as a complete application with everything you need to create an application.

## How to use it

Your Dockerfile must include the Query Server. The Dockerfile could be a multistage one, where the last stage should be an `x86_64-unknown-linux-gnu` compatible image. We recommend using a `debian:<suite>-slim` image.

Please refer to the [LiteFS documentation](https://fly.io/docs/litefs/speedrun/) for more information, as it is a crucial system component.

Dockerfile:

```sh
FROM debian:12-slim

COPY litefs.yml /etc/litefs.yml
COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs

RUN apt-get update -qq && \
 apt-get install -y --no-install-recommends \
 ca-certificates \
 sqlite3 \
 fuse3 \
 curl

# Download and install Query Server
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-server-installer.sh | sh

# It will execute the Query Server and your App
COPY process.sh process.sh
RUN chmod +x process.sh

# Path to the Query databases
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

## Fly configuration

If this is your first time using Fly, you can follow the [Quick Start](https://fly.io/docs/getting-started/launch/) guide to install the CLI, sign up, and sign in.

Once you have the Fly CLI installed, you have to rename the `fly.toml.dist` to `fly.toml` and update it with your app name and the primary region running the following command:

```sh
fly launch
```

It is time to set the environment variables for your app. You can do it by running the following commands:

Token secret:

```sh
fly secrets set QUERY_SERVER_TOKEN_SECRET=$(openssl rand -hex 32)
```

> **Tip**: If you don't have openssl installed, you can also use
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

We are using LiteFS, a distributed file system that transparently replicates SQLite databases. It offers a straightforward way to replicate your SQLite databases in the cloud.

To use LiteFS, you will need to configure Consul. You can do it by running the following command:

```sh
fly consul attach
```

Then you can deploy your app running:

```sh
fly deploy
```

Your app is currently running on a single machine. To ensure high availability, especially for production apps, Fly strongly recommends running at least 2 instances. You can scale up the number of machines using the `fly machine clone` command in the CLI. Please keep in mind that you can add that machine to another region.

```sh
fly m clone
```

Or

```sh
fly m clone --select --region A_REGION
```

*Example: `fly m clone --select --region lhr`* (London)

To get a list of regions, you can run the following command:

```sh
fly platform regions
```
