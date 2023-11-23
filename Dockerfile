# syntax=docker/dockerfile:1.3
FROM rust:1 AS builder
WORKDIR /root

# Deno prerequisites
# https://docs.deno.com/runtime/manual/references/contributing/building_from_source#native-compilers-and-linkers
RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends cmake

RUN --mount=type=cache,target=/usr/local/cargo/registry
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    RUST_LOG=trace cargo build --package query-server --release && \
    mv /root/target/release/query-server /root

FROM debian:12-slim AS runtime

COPY --from=builder /root/query-server /

ADD litefs.yml /etc/litefs.yml
COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs

RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    sqlite3 \
    fuse3 \
    curl

CMD ["litefs", "mount"]

EXPOSE 3000
