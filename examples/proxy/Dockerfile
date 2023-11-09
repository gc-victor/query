# syntax=docker/dockerfile:1

FROM oven/bun as base

# Remix app lives here
WORKDIR /app

# Set production environment
ENV NODE_ENV="production"

# Throw-away build stage to reduce size of final image
FROM base as build

# Install node modules
COPY --link bun.lockb package.json ./
RUN bun install --ci

# Copy application code
COPY --link . .

FROM debian:12-slim AS runtime

ADD litefs.yml /etc/litefs.yml
COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs

RUN apt-get update -qq && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    sqlite3 \
    fuse3 \
    unzip \
    curl

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/query/releases/latest/download/query-server-installer.sh | sh

RUN curl -fsSL https://bun.sh/install | bash

COPY --link process.sh process.sh
COPY --link index.ts /app/index.ts
COPY --link tsconfig.json /app/tsconfig.json
COPY --link package.json /app/package.json
COPY --from=build /app /app
COPY --from=build /app/node_modules /app/node_modules

RUN chmod +x process.sh

ENV QUERY_SERVER_PORT="3000"
ENV QUERY_SERVER_PROXY="true"
ENV QUERY_SERVER_PROXY_PORT="3001"
ENV QUERY_SERVER_DBS_PATH="/mnt/dbs"

ENV QUERY_SERVER_TOKEN_SECRET="secret"
ENV QUERY_SERVER_ADMIN_EMAIL="admin"
ENV QUERY_SERVER_ADMIN_PASSWORD="admin"

EXPOSE 3000

CMD ["litefs", "mount"]
