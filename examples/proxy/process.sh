#!/bin/bash

# https://fly.io/docs/app-guides/multiple-processes/#just-use-bash
set -m
~/.cargo/bin/query-server &
~/.bun/bin/bun /app/index.ts &
fg %1