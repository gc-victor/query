# Server Proxy

Query allows you to set a proxy to an App in the same VM. It provides you access to the databases directly from your application while enjoying the benefits of using Query, such as the server, the CLI, the API or [Query Studio](https://github.com/gc-victor/query-studio), for example.

> This addresses one of the common concerns people have with SQLite. Very interesting idea! And there's an [Epic Stack example](https://github.com/gc-victor/epic-stack-with-query) too!
>
> [Kent C. Dodds](https://twitter.com/kentcdodds/status/1729133823039045994)

## How to use it

In your Dockerfile, you must include the Query Server and your Application together. The Dockerfile could be a multistage one, where the last stage should be an `x86_64-unknown-linux-gnu` compatible image. We recommend using a `debian:<suite>-slim` image.

Please refer to the [LiteFS documentation](https://fly.io/litefs/speedrun/) for more information, as it is a crucial system component.

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

For multi-process applications, you can use the "Just use Bash", as we do in this example, to start the Query Server and your App. [Fly proposes](https://fly.io/app-guides/multiple-processes/) different ways to manage multiple processes, so please use the one you feel more comfortable with.

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
