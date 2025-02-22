# Changelog

All notable changes to this project will be documented in this file.

## [0.12.48] - 2025-02-22

### Build

- *(deps)* Rollback watchexec to v5.0

## [0.12.47] - 2025-02-22

### 🐛 Bug Fixes

- *(jsx_parser)* Jsx children

### Build

- *(deps)* Bump tempfile from 3.15.0 to 3.16.0
- *(deps)* Bump lettre from 0.11.11 to 0.11.12
- *(deps)* Bump webpki-roots from 0.26.7 to 0.26.8
- *(deps)* Bump ryu from 1.0.18 to 1.0.19
- *(deps)* Bump toml_edit from 0.22.22 to 0.22.23
- *(deps)* Bump openssl from 0.10.69 to 0.10.70
- *(deps)* Update rquickjs & llrt
- *(deps)* Bump watchexec from 5.0.0 to 6.0.0
- *(deps)* Bump once_cell from 1.20.2 to 1.20.3
- *(deps)* Bump tabled from 0.17.0 to 0.18.0
- *(deps)* Bump jsonwebtoken from 9.3.0 to 9.3.1
- *(deps)* Bump ring from 0.17.8 to 0.17.9
- *(deps)* Bump toml_edit from 0.22.23 to 0.22.24
- *(deps)* Bump extism from 1.9.1 to 1.10.0
- *(deps)* Bump lettre from 0.11.12 to 0.11.13
- Update dist 0.27.1 to 0.28.0
- *(deps)* Bump esbuild to 0.25.0

## [0.12.46] - 2025-01-31

### 🐛 Bug Fixes

- *(jsx_parser)* Skip whitespace
- Clippy

## [0.12.45] - 2025-01-29

### 🐛 Bug Fixes

- *(jsx_parser)* Attribute single quote

### Build

- *(deps)* Bump openssl from 0.10.68 to 0.10.69
- *(deps)* Bump rand from 0.8.5 to 0.9.0
- *(deps)* Bump uuid from 1.12.0 to 1.12.1
- *(deps)* Bump clap from 4.5.26 to 4.5.27

## [0.12.44] - 2025-01-26

### 🐛 Bug Fixes

- Clippy
- *(jsx_parser)* Jsx extract web components

### 📚 Documentation

- Replace #4ade80 by #fff

### Build

- *(deps)* Bump uuid from 1.11.0 to 1.12.0
- *(deps)* Bump phf from 0.11.2 to 0.11.3
- *(deps)* Bump rusqlite from 0.32.1 to 0.33.0
- *(deps)* Bump serde_json from 1.0.134 to 1.0.137
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps)* Bump valibot in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps)* Bump rquickjs from `ff980e2` to `a39e7b9`

## [0.12.43] - 2025-01-17

### 🐛 Bug Fixes

- *(jsx_parser)* Handle nested JSX in fragment expressions

### Build

- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps-dev)* Bump typescript in /examples/minimal
- *(deps)* Bump valibot in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump update-browserslist-db in /examples/application
- *(deps-dev)* Bump typescript in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump update-browserslist-db in /examples/minimal
- *(deps)* Bump rustls from 0.23.20 to 0.23.21
- *(deps-dev)* Bump typescript in /examples/application
- *(deps)* Bump clap from 4.5.23 to 4.5.26
- *(deps)* Bump tempfile from 3.14.0 to 3.15.0
- *(deps)* Bump tokio from 1.42.0 to 1.43.0
- *(deps)* Bump colored from 2.2.0 to 3.0.0
- *(deps)* Bump llrt version to 92a053f

## [0.12.42] - 2025-01-10

### 🐛 Bug Fixes

- *(server)* Bind to params
- *(runtime)* Bind to params
- Clippy

## [0.12.41] - 2025-01-09

### 🐛 Bug Fixes

- *(runtime)* Fetcher accept header

### Build

- Update dist 0.24.1 to 0.27.0
- Update dist 0.27.0 to 0.27.1
- *(deps)* Bump rquickjs from `832be2f` to `ff980e2`
- *(deps)* Bump reqwest from 0.12.9 to 0.12.12
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps)* Bump valibot in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/counter

## [0.12.40] - 2025-01-03

### 🚀 Features

- *(docs)* Update cli test api
- *(docs)* Update runtime-compat
- *(runtime)* Add llrt timers module
- *(docs)* Update runtime-compat

## [0.12.39] - 2024-12-31

### 🚀 Features

- *(runtime)* Add llrt crypto module

### Build

- *(deps)* Bump serde from 1.0.216 to 1.0.217
- *(deps)* Bump anyhow from 1.0.93 to 1.0.95
- *(deps)* Bump hyper from 1.5.1 to 1.5.2
- *(deps)* Bump hyper-rustls from 0.27.3 to 0.27.5
- *(deps)* Bump serde_json from 1.0.133 to 1.0.134
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump tailwindcss in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump esbuild from 0.24.0 to 0.24.2 in /examples/minimal
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump esbuild from 0.24.0 to 0.24.2 in /examples/counter
- *(deps-dev)* Bump tailwindcss in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump tailwindcss in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps-dev)* Bump esbuild in /examples/application

## [0.12.38] - 2024-12-21

### 🚀 Features

- *(runtime)* Update rquickjs to version 0.8.1
- *(runtime)* Add lifecycle hooks

### 🐛 Bug Fixes

- Esbuild regression (pin to 0.24.0)

### 📚 Documentation

- Add test runner lifecycle hooks

## [0.12.37] - 2024-12-20

### 🐛 Bug Fixes

- *(cli)* Function esbuild external

## [0.12.36] - 2024-12-19

### 🐛 Bug Fixes

- *(jsx_parser)* Empty spread with spaces

## [0.12.35] - 2024-12-18

### 🐛 Bug Fixes

- *(jsx_parser)* Self closing component without spaces

## [0.12.34] - 2024-12-18

### 🚀 Features

- *(example)* Update to jsx preserve
- *(docs)* Add jsx server-side
- Add test runner

### 🐛 Bug Fixes

- *(jsx_parser)* Child spaces and bool attr
- *(runtime)* Return value accepts args

### 📚 Documentation

- Add cli test runner

### Build

- *(deps)* Bump tabled from 0.16.0 to 0.17.0
- *(deps)* Bump clap from 4.5.21 to 4.5.23
- *(deps)* Bump rustyline from 14.0.0 to 15.0.0
- *(deps)* Bump chrono from 0.4.38 to 0.4.39
- *(deps)* Bump tokio from 1.41.1 to 1.42.0
- *(deps)* Bump lettre from 0.11.10 to 0.11.11
- *(deps)* Bump colored from 2.1.0 to 2.2.0
- *(deps)* Bump rustls from 0.23.19 to 0.23.20
- *(deps)* Bump serde from 1.0.215 to 1.0.216
- *(deps)* Bump time from 0.3.36 to 0.3.37

## [0.12.33] - 2024-12-04

### 🐛 Bug Fixes

- *(jsx_parser)* Component spread

## [0.12.32] - 2024-12-03

### 🚀 Features

- *(server)* Move function and function_builder
- *(server)* Builder query cache invalidate

## [0.12.31] - 2024-12-03

### 🚀 Features

- *(ci)* Ignore paths to run tests
- *(ci)* Ignore paths to run tests
- Add dual license
- *(ci)* Update dist-workspace on prerelease
- *(ci)* Change dist-workspace target on prerelease
- *(server)* Add content cache
- *(server)* Add cache invalidation
- *(server)* Remove cache function
- *(server)* Remove purge
- *(server)* Optimize cache invalidation
- *(ci)* Add dev-server
- *(server)* Optimize sqlite
- Jsx parser

### 🐛 Bug Fixes

- *(ci)* Hurl skip email test
- *(docs)* Install mdbook fixed version
- *(cli)* Current running executable
- *(server)* Invalidate cache

### 📚 Documentation

- Add functions to modules
- Update cache references
- Remove purge references

### Build

- Update cargo dist 0.22.1 to 0.23.0
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/minimal (#418)
- *(deps)* Bump openssl from 0.10.66 to 0.10.68 (#406)
- *(deps)* Bump extism-manifest from 1.7.0 to 1.8.0
- *(deps-dev)* Bump @biomejs/biome in /examples/minimal
- *(deps)* Bump simd-json from 0.14.1 to 0.14.2
- *(deps)* Bump regex from 1.11.0 to 1.11.1
- *(deps)* Bump bytes from 1.7.2 to 1.8.0
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump @biomejs/biome in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @biomejs/biome in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump tailwindcss in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/application (#423)
- Update dist 0.23.0 to 0.24.0
- Update dist 0.24.0 to 0.24.1
- *(deps)* Bump hyper-util from 0.1.9 to 0.1.10
- *(deps)* Bump reqwest from 0.12.8 to 0.12.9
- *(deps)* Bump lettre from 0.11.9 to 0.11.10
- *(deps)* Bump extism from 1.7.0 to 1.8.0
- *(deps)* Bump openssl from 0.10.66 to 0.10.68
- *(deps)* Bump serde from 1.0.210 to 1.0.215
- *(deps)* Bump simd-json from 0.14.2 to 0.14.3
- *(deps)* Bump rustls from 0.23.15 to 0.23.17
- *(deps)* Bump serde_json from 1.0.132 to 1.0.133
- *(deps)* Bump clap from 4.5.20 to 4.5.21
- *(deps-dev)* Bump tailwindcss in /examples/counter
- *(deps-dev)* Bump tailwindcss in /examples/application
- *(deps)* Bump cross-spawn from 7.0.3 to 7.0.6 in /examples/minimal
- *(deps)* Bump cross-spawn from 7.0.3 to 7.0.6 in /examples/counter
- *(deps)* Bump extism from 1.8.0 to 1.9.0
- *(deps)* Bump hyper from 1.5.0 to 1.5.1
- *(deps)* Bump tokio from 1.40.0 to 1.41.1
- *(deps)* Bump rustls from 0.23.17 to 0.23.18

## [0.12.30] - 2024-10-22

### 🐛 Bug Fixes

- *(runtime)* Add attachment and file inline

### 📚 Documentation

- Create modules

## [0.12.29] - 2024-10-22

### 🚀 Features

- *(example)* Update config
- *(ci)* Update dist fly config
- *(runtime)* Add email send
- *(ci)* Update esbuild version
- *(ci)* Update hurl to version 5.0.1
- *(dist)* Remove powershell

### 🐛 Bug Fixes

- *(docs)* Replace broken link

### Build

- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps)* Bump preact from 10.24.1 to 10.24.2 in /examples/counter
- *(deps-dev)* Bump @biomejs/biome in /examples/counter
- *(deps)* Bump futures-util from 0.3.30 to 0.3.31
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps)* Bump preact in /examples/application
- *(deps-dev)* Bump @biomejs/biome in /examples/application
- *(deps)* Bump watchexec from 4.1.0 to 5.0.0
- *(deps)* Bump clap from 4.5.19 to 4.5.20
- *(deps)* Bump simd-json from 0.14.0 to 0.14.1
- *(deps)* Bump regex from 1.10.6 to 1.11.0
- *(deps)* Bump hyper from 1.4.1 to 1.5.0
- *(deps)* Bump rustls from 0.23.14 to 0.23.15
- *(deps)* Bump uuid from 1.10.0 to 1.11.0
- *(deps)* Bump anyhow from 1.0.89 to 1.0.90
- *(deps)* Bump serde_json from 1.0.128 to 1.0.132
- *(deps)* Bump braces from 3.0.2 to 3.0.3 in /examples/counter
- *(deps)* Bump preact from 10.24.2 to 10.24.3 in /examples/counter
- *(deps-dev)* Bump typescript in /examples/counter
- *(deps)* Bump preact in /examples/application
- *(deps-dev)* Bump typescript in /examples/application

### Dev

- *(deps)* Update minimal example dependencies

## [0.12.28] - 2024-10-11

### 🚀 Features

- Add deploy command

### Build

- *(deps)* Bump once_cell from 1.20.1 to 1.20.2
- *(deps)* Bump clap from 4.5.18 to 4.5.19
- *(deps)* Bump reqwest from 0.12.7 to 0.12.8
- *(deps)* Bump rustls from 0.23.13 to 0.23.14

## [0.12.27] - 2024-10-07

### 🚀 Features

- *(example)* Minimal example enhanced

### 🐛 Bug Fixes

- *(cli)* Esbuild flags without value

## [0.12.26] - 2024-10-02

### 🐛 Bug Fixes

- *(cli)* Watch on macos

### 📚 Documentation

- Fix edited commit

### Build

- *(deps)* Bump preact in /examples/application
- *(deps)* Bump preact from 10.24.0 to 10.24.1 in /examples/counter
- *(deps-dev)* Bump update-browserslist-db in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query in /examples/application

## [0.12.25] - 2024-09-30

### 🐛 Bug Fixes

- *(runtime)* Get form data empty fields
- *(runtime)* Form data names with brackets

### Build

- *(deps)* Bump extism from 1.6.0 to 1.7.0
- *(deps)* Bump simd-json from 0.13.10 to 0.14.0
- *(deps)* Bump toml_edit from 0.22.20 to 0.22.22
- *(deps)* Bump hyper-util from 0.1.8 to 0.1.9
- *(deps)* Bump once_cell from 1.20.0 to 1.20.1
- Update cargo dist 0.19.1 to 0.22.1

## [0.12.24] - 2024-09-27

### 🐛 Bug Fixes

- *(docs)* Add generate
- *(runtime)* Request to form data
- *(cli)* Clippy updates

### Build

- *(deps)* Bump preact-render-to-string in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump typescript in /examples/minimal
- *(deps)* Bump preact from 10.23.2 to 10.24.0 in /examples/counter
- *(deps-dev)* Bump typescript in /examples/counter
- *(deps)* Bump anyhow from 1.0.86 to 1.0.89
- *(deps)* Bump once_cell from 1.19.0 to 1.20.0
- *(deps)* Bump rustls from 0.23.12 to 0.23.13
- *(deps)* Bump serde_json from 1.0.127 to 1.0.128
- *(deps)* Bump cliclack from 0.3.4 to 0.3.5
- *(deps)* Bump valibot from 0.41.0 to 0.42.0 in /examples/application
- *(deps)* Bump preact in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps)* Bump preact-render-to-string in /examples/counter
- *(deps-dev)* Bump typescript in /examples/application
- *(deps)* Bump webpki-roots from 0.26.5 to 0.26.6
- *(deps)* Bump hyper-rustls from 0.27.2 to 0.27.3
- *(deps)* Bump clap from 4.5.17 to 4.5.18
- *(deps)* Bump bytes from 1.7.1 to 1.7.2
- *(deps-dev)* Bump esbuild from 0.23.1 to 0.24.0 in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump @biomejs/biome in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump tailwindcss in /examples/counter
- *(deps)* Bump valibot from 0.42.0 to 0.42.1 in /examples/application
- *(deps)* Bump extism from 1.5.0 to 1.6.0
- *(deps-dev)* Bump esbuild from 0.23.1 to 0.24.0 in /examples/counter
- *(deps-dev)* Bump @biomejs/biome in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump tailwindcss in /examples/application
- *(deps-dev)* Bump @biomejs/biome in /examples/application
- *(deps-dev)* Bump esbuild in /examples/application

## [0.12.23] - 2024-09-13

### 🚀 Features

- *(doc)* Enhance docs for the web
- *(runtime)* Import database
- *(ci)* Remove pull-request
- *(docs)* New readme.md version

### 🐛 Bug Fixes

- *(example)* Add task to minimal
- *(docs)* Update broken link server-proxy.md
- *(ci)* Docs search

### Build

- *(deps)* Bump clap from 4.5.15 to 4.5.16
- *(deps)* Bump serde from 1.0.207 to 1.0.208
- *(deps)* Bump serde_json from 1.0.124 to 1.0.125
- *(deps)* Bump tabled from 0.15.0 to 0.16.0
- *(deps)* Bump tokio from 1.39.2 to 1.39.3
- *(deps)* Bump preact in /examples/application
- *(deps-dev)* Bump esbuild in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump tailwindcss in /examples/application
- *(deps-dev)* Bump tailwindcss in /examples/counter
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump esbuild from 0.23.0 to 0.23.1 in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump esbuild from 0.21.2 to 0.23.1 in /examples/minimal
- *(deps)* Bump quinn-proto from 0.11.6 to 0.11.8
- *(deps)* Bump tokio from 1.39.3 to 1.40.0
- *(deps)* Bump serde from 1.0.208 to 1.0.209
- *(deps)* Bump serde_json from 1.0.125 to 1.0.127
- *(deps)* Bump reqwest from 0.12.5 to 0.12.7
- *(deps)* Bump cliclack from 0.3.3 to 0.3.4
- *(deps)* Bump preact-render-to-string in /examples/counter
- *(deps)* Bump preact-render-to-string in /examples/application
- *(deps)* Bump webpki-roots from 0.26.3 to 0.26.5
- *(deps)* Bump serde from 1.0.209 to 1.0.210
- *(deps)* Bump clap from 4.5.16 to 4.5.17
- *(deps)* Bump hyper-util from 0.1.7 to 0.1.8
- *(deps)* Bump extism-manifest from 1.5.0 to 1.6.0
- *(deps-dev)* Bump typescript in /examples/minimal
- *(deps-dev)* Bump @biomejs/biome in /examples/minimal
- *(deps)* Bump valibot from 0.37.0 to 0.41.0 in /examples/application

## [0.12.22] - 2024-08-16

### 🐛 Bug Fixes

- *(cli)* Typo command/s
- *(cli)* Add toml feature preserve order
- *(cli)* Add run task asset public
- *(cli)* Remove output conditions

## [0.12.21] - 2024-08-15

### 🚀 Features

- *(cli)* Add env query_cli_dev

### 🐛 Bug Fixes

- *(ci)* Print release commit
- *(cli)* Enhance logs

## [0.12.20] - 2024-08-15

### 🚀 Features

- *(cli)* Allow no task
- *(example)* Use pnpm lock
- Add plugin system
- *(ci)* Add release

### 🐛 Bug Fixes

- *(server)* Execute timers
- *(cli)* Enhance logs

### 📚 Documentation

- Changelog version 0.12.19

### ⚙️ Miscellaneous Tasks

- Add hurl function

### Build

- *(deps)* Bump serde_json from 1.0.121 to 1.0.122
- *(deps)* Bump toml from 0.8.16 to 0.8.19
- *(deps)* Bump clap from 4.5.11 to 4.5.13
- *(deps)* Bump bytes from 1.6.1 to 1.7.1
- *(deps)* Bump regex from 1.10.5 to 1.10.6
- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps)* Bump valibot from 0.36.0 to 0.37.0 in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/counter
- *(deps-dev)* Bump @qery/query in /examples/minimal
- *(deps-dev)* Bump @qery/query-server in /examples/application
- *(deps-dev)* Bump @qery/query-server in /examples/minimal
- *(deps)* Bump cliclack from 0.3.2 to 0.3.3
- *(deps-dev)* Bump tailwindcss in /examples/application
- *(deps)* Bump serde_json from 1.0.122 to 1.0.124
- *(deps)* Bump hyper-util from 0.1.6 to 0.1.7
- *(deps)* Bump serde from 1.0.204 to 1.0.206
- *(deps)* Bump clap from 4.5.13 to 4.5.15
- *(deps)* Bump @preact/signals in /examples/counter
- *(deps)* Bump preact-render-to-string in /examples/application
- *(deps)* Bump preact from 10.22.0 to 10.23.2 in /examples/counter
- *(deps)* Bump serde from 1.0.206 to 1.0.207
- *(deps-dev)* Bump tailwindcss in /examples/counter
- *(deps)* Bump preact-render-to-string in /examples/counter

## [0.12.19] - 2024-08-02

### 🐛 Bug Fixes

- *(cli)* Has node_modules binary

### 📚 Documentation

- Changelog version 0.12.18

### Release

- Version 0.12.19

## [0.12.18] - 2024-07-31

### 🚀 Features

- *(example)* Update application deploy workflow
- *(ci)* Add examples dependabot
- *(docs)* Update create repo url

### 🐛 Bug Fixes

- *(server)* Revert rutime cache (#225)

### 📚 Documentation

- Changelog version 0.12.17

### Build

- *(deps-dev)* Bump @qery/query in /examples/application
- *(deps-dev)* Bump typescript in /examples/counter
- *(deps-dev)* Bump esbuild from 0.21.5 to 0.23.0 in /examples/counter
- *(deps-dev)* Bump @biomejs/biome in /examples/counter
- *(deps)* Bump toml from 0.8.15 to 0.8.16
- *(deps)* Bump rustls from 0.23.11 to 0.23.12
- *(deps-dev)* Bump @qery/query-server in /examples/application (#224)

### Release

- Version 0.12.18

## [0.12.17] - 2024-07-30

### 🚀 Features

- Runtime cache
- *(cli)* Add create repository
- *(example)* Update application

### 📚 Documentation

- Changelog version 0.12.16

### Build

- *(deps)* Bump tokio from 1.38.1 to 1.39.2
- *(deps)* Bump clap from 4.5.9 to 4.5.11
- *(deps)* Bump rusqlite from 0.32.0 to 0.32.1
- *(deps)* Bump toml_edit from 0.22.16 to 0.22.17
- *(deps)* Bump serde_json from 1.0.120 to 1.0.121

### Release

- Version 0.12.17

## [0.12.16] - 2024-07-25

### 🚀 Features

- *(runtime)* Llrt updates

### 🐛 Bug Fixes

- Rollback readme
- *(server)* Memory

### 📚 Documentation

- Changelog version 0.12.15

### Build

- *(deps)* Bump openssl from 0.10.64 to 0.10.66
- *(deps)* Bump tokio from 1.38.0 to 1.38.1
- *(deps)* Bump toml from 0.8.14 to 0.8.15
- *(deps)* Bump rusqlite from 0.31.0 to 0.32.0
- *(deps)* Bump rustls from 0.23.10 to 0.23.11
- *(deps)* Bump time from 0.3.34 to 0.3.36

### Release

- Version 0.12.16

## [0.12.15] - 2024-07-20

### 🚀 Features

- *(server)* Speed up runtime

### 📚 Documentation

- Changelog version 0.12.14
- Remove litefs cloud

### ⚙️ Miscellaneous Tasks

- Update help

### Build

- *(deps)* Update dependencies

### Release

- Version 0.12.15

## [0.12.14] - 2024-07-17

### 🐛 Bug Fixes

- *(runtime)* Parse multipart

### 📚 Documentation

- Changelog version 0.12.13

### Dev

- *(deps)* Update @biomejs/biome from 1.8.0 to 1.8.3

### Release

- Version 0.12.14

## [0.12.13] - 2024-07-17

### 🚀 Features

- *(server)* Optimize cache function deletion and creation
- *(server)* Path as module name
- *(runtime)* Llrt updates
- *(server)* Remove rquickjs patch
- *(server)* Handle functions

### 🐛 Bug Fixes

- *(runtime)* Convert args to string

### 🚜 Refactor

- *(cli)* Add clippy fixes

### 📚 Documentation

- Changelog versions to 0.12.12

### ⚙️ Miscellaneous Tasks

- Add help

### Build

- *(deps)* Bump serde from 1.0.203 to 1.0.204
- *(deps)* Bump serde_json from 1.0.117 to 1.0.120
- *(deps)* Bump clap from 4.5.7 to 4.5.8
- *(deps)* Bump toml_edit from 0.22.14 to 0.22.15
- *(deps)* Bump clap from 4.5.8 to 4.5.9
- *(deps)* Bump hyper from 1.4.0 to 1.4.1
- *(deps)* Bump uuid from 1.9.1 to 1.10.0
- Update cargo dist 0.18.0 to 0.19.1

### Release

- Version 0.12.13

## [0.12.12] - 2024-07-07

### 🚀 Features

- *(runtime)* Enhance console

### 📚 Documentation

- Changelog versions to 0.12.11

### ⚙️ Miscellaneous Tasks

- Update changelog

### Release

- Version 0.12.12

## [0.12.11] - 2024-07-04

### 🚀 Features

- *(server)* Add transaction immediate

### 🐛 Bug Fixes

- *(server)* Parse bad control characters

### Build

- *(deps)* Bump rustls from 0.23.9 to 0.23.10
- *(deps)* Bump http-body-util from 0.1.1 to 0.1.2
- *(deps)* Bump reqwest from 0.12.4 to 0.12.5
- *(deps)* Bump uuid from 1.8.0 to 1.9.0
- *(deps)* Bump lazy_static from 1.4.0 to 1.5.0
- *(deps)* Bump webpki-roots from 0.26.2 to 0.26.3
- *(deps)* Bump url from 2.5.1 to 2.5.2
- *(deps)* Bump hyper from 1.3.1 to 1.4.0
- *(deps)* Bump uuid from 1.9.0 to 1.9.1
- *(deps)* Bump serde_bytes from 0.11.14 to 0.11.15
- *(deps)* Bump mime_guess from 2.0.4 to 2.0.5
- *(deps)* Bump hyper-util from 0.1.5 to 0.1.6
- Update cargo dist 0.15.1 to 0.18.0

### Release

- Version 0.12.11

## [0.12.10] - 2024-06-17

### 🐛 Bug Fixes

- *(server)* Add missing escape double quotes
- *(runtime)* Remove console escaped double quotes

### Release

- Version 0.12.10

## [0.12.9] - 2024-06-14

### 🐛 Bug Fixes

- *(server)* Multipart form-data fields with same name

### ⚙️ Miscellaneous Tasks

- Add build server watch

### Release

- Version 0.12.9

## [0.12.8] - 2024-06-12

### 🐛 Bug Fixes

- Hurl tests
- *(server)* Handle multiple form data fields with identical names

### Build

- *(deps)* Bump regex from 1.10.4 to 1.10.5
- *(deps)* Bump toml from 0.8.13 to 0.8.14
- *(deps)* Bump url from 2.5.0 to 2.5.1
- *(deps)* Bump clap from 4.5.4 to 4.5.7
- *(deps)* Bump webpki-roots from 0.26.1 to 0.26.2

### Release

- Version 0.12.8

## [0.12.7] - 2024-06-05

### 🚀 Features

- *(cli)* Add generate foreign uuid

### 🧪 Testing

- *(cli)* Add missing foreign test

### Build

- *(deps)* Bump rustls from 0.23.8 to 0.23.9
- *(deps)* Bump hyper-util from 0.1.4 to 0.1.5
- *(deps)* Bump tokio from 1.37.0 to 1.38.0
- Update cargo dist 0.15.0 to 0.15.1

### Dev

- *(deps)* Update @biomejs/biome from 1.6.4 to 1.8.0

### Release

- Version 0.12.7

## [0.12.6] - 2024-06-02

### 🚀 Features

- *(example)* Add table min-w-full
- *(cli)* Add generate foreign

### 🐛 Bug Fixes

- *(example)* Execute sh in the background
- *(example)* Add missing fragments
- *(example)* Add missing key
- *(example)* Replace interface semicolons by commas
- *(example)* Add missing keys

### 📚 Documentation

- Changelog version 0.12.5

### Build

- *(deps)* Bump rustls from 0.23.5 to 0.23.7
- *(deps)* Bump toml from 0.8.12 to 0.8.13
- *(deps)* Bump anyhow from 1.0.83 to 1.0.86
- *(deps)* Bump toml_edit from 0.22.12 to 0.22.13
- *(deps)* Bump serde from 1.0.201 to 1.0.202
- *(deps)* Bump serde from 1.0.202 to 1.0.203
- *(deps)* Bump rustls from 0.23.7 to 0.23.8
- *(deps)* Bump hyper-rustls from 0.27.1 to 0.27.2
- *(deps)* Bump hyper-util from 0.1.3 to 0.1.4
- Update cargo dist 0.14.1 to 0.15.0

### Release

- Version 0.12.6

## [0.12.5] - 2024-05-15

### 🚀 Features

- *(cli)* Cache dev commands
- *(cli)* Update query create
- *(cli)* Remove tailwind-css dumb error
- *(example)* Update application tasks
- *(example)* Update application dependencies
- *(example)* Create counter
- Create examples

### 🐛 Bug Fixes

- *(cli)* Revert cache modified time
- Replace template placeholder

### 🚜 Refactor

- Add clippy fixes

### 📚 Documentation

- Update changelog
- Add quick start
- Remove query dev no-port-check

### 🎨 Styling

- Update format

### Build

- *(deps)* Bump serde from 1.0.200 to 1.0.201
- *(deps)* Bump serde_json from 1.0.116 to 1.0.117
- *(deps)* Bump ryu from 1.0.17 to 1.0.18
- *(deps)* Bump anyhow from 1.0.82 to 1.0.83

### Release

- Version 0.12.5

## [0.12.4] - 2024-05-09

### 🚀 Features

- *(cli)* Update token
- *(cli)* Update user-token
- *(cli)* Update user
- *(cli)* Update branch

### Build

- Update cargo dist 0.14.0 to 0.14.1

### Release

- Version 0.12.4

## [0.12.3] - 2024-05-08

### 🚀 Features

- *(cli)* Announce running initial tasks

### 🐛 Bug Fixes

- *(cli)* Typo query task create
- *(cli)* Add toml_edit
- *(cli)* Dev server error logs

### Release

- Version 0.12.3

## [0.12.2] - 2024-05-08

### 🚀 Features

- *(cli)* Remove no_port_check flag
- *(cli)* Update settings

### 🐛 Bug Fixes

- *(cli)* Skip run server without install dependencies
- *(cli)* Enhance final message
- *(cli)* Avoid query_server logs
- *(cli)* Query create npx final message

### 📚 Documentation

- Add query create

### Release

- Version 0.12.2

## [0.12.1] - 2024-05-07

### 🐛 Bug Fixes

- *(cli)* Block timeout
- *(cli)* Run task create

### Release

- Version 0.12.1

## [0.12.0] - 2024-05-07

### 🚀 Features

- *(cli)* Update query task cli docs
- *(cli)* Add query create

### 🐛 Bug Fixes

- Avoid watchexec logs
- *(cli)* List all tasks
- *(cli)* Execute none table tasks
- *(cli)* Cache file_hash for assets
- *(cli)* Watch dist folder
- *(cli)* Enhance log messages

### 📚 Documentation

- Add changelog
- Add query task
- Update query dev

### Build

- *(deps)* Bump rquickjs from 0.6.0 to 0.6.2
- *(deps)* Bump serde from 1.0.199 to 1.0.200
- Update cargo dist 0.13.3 to 0.14.0

### Release

- Version 0.12.0

## [0.11.0] - 2024-04-30

### 🚀 Features

- *(cli)* Add query task

### ⚙️ Miscellaneous Tasks

- Update npm publish and unpublish prerelease

### Build

- *(deps)* Bump watchexec from 4.0.0 to 4.1.0
- *(deps)* Bump chrono from 0.4.37 to 0.4.38
- *(deps)* Bump serde from 1.0.198 to 1.0.199
- *(deps)* Bump inquire from 0.7.4 to 0.7.5

### Release

- Version 0.11.0

## [0.10.0] - 2024-04-27

### 🚀 Features

- *(cli)* Add query purge
- *(cli)* Cache modified time
- *(cli)* Update query dev

### Release

- Version 0.10.0

## [0.9.0] - 2024-04-25

### 🚀 Features

- *(runtime)* Stringify console object

### 🐛 Bug Fixes

- *(server)* Cache function path

### ⚙️ Miscellaneous Tasks

- Manage npm publish

### Build

- Update cargo dist 0.13.0 to 0.13.3
- *(deps)* Bump rquickjs 0.5.1 to 0.6.0

### Release

- Version 0.9.0

## [0.8.1] - 2024-04-24

### 🐛 Bug Fixes

- *(cli)* Pnpm query dev

### 📚 Documentation

- Update dev mode

### Release

- Version 0.8.1

## [0.8.0] - 2024-04-24

### 🚀 Features

- *(cli)* Add settings default url
- *(cli)* Add query dev

### 🐛 Bug Fixes

- *(cli)* Update token active prompt

### 🚜 Refactor

- *(cli)* Add clippy fixes

### 📚 Documentation

- Add dev mode
- Add npm package install

### Build

- *(deps)* Bump reqwest from 0.11.27 to 0.12.3
- *(deps)* Bump hyper from 1.2.0 to 1.3.0
- *(deps)* Bump hyper-rustls from 0.27.0 to 0.27.1
- *(deps)* Bump anyhow from 1.0.81 to 1.0.82
- *(deps)* Bump rustls from 0.23.4 to 0.23.5
- *(deps)* Bump h2 from 0.4.3 to 0.4.4
- *(deps)* Bump hyper from 1.3.0 to 1.3.1
- *(deps)* Bump serde_json from 1.0.115 to 1.0.116
- *(deps)* Bump simd-json from 0.13.9 to 0.13.10
- *(deps)* Bump reqwest from 0.12.3 to 0.12.4
- *(deps)* Bump serde from 1.0.197 to 1.0.198

### Release

- Version 0.8.0

## [0.7.0] - 2024-04-16

### 🚀 Features

- Add function caching

### Release

- Version 0.7.0

## [0.6.0] - 2024-04-12

### 🚀 Features

- Create js runtime

### Build

- Update cargo dist 0.11.1 to 0.13.0
- Update tests action to use Node.js 20

### Release

- Version 0.6.0

## [0.5.4] - 2024-03-25

### 🐛 Bug Fixes

- Set password for user token value

### Release

- Version 0.5.4

## [0.5.3] - 2024-03-22

### 🚀 Features

- *(cli)* Asset and function error log
- *(server)* Remove panic from dbs creation

### Build

- *(deps)* Bump toml from 0.8.10 to 0.8.11
- *(deps)* Bump anyhow from 1.0.80 to 1.0.81
- *(deps)* Bump inquire from 0.7.0 to 0.7.2
- *(deps)* Bump clap from 4.5.1 to 4.5.3

### Release

- Version 0.5.3

## [0.5.2] - 2024-03-17

### 🐛 Bug Fixes

- Libssl version

### 📚 Documentation

- Update dockerfile example

### Release

- Version 0.5.2

## [0.5.1] - 2024-03-06

### 🚀 Features

- *(cli)* Use snake case for table name

### 📚 Documentation

- Fix column type typo
- Fix dynamic variables style

### Build

- *(deps)* Bump mio from 0.8.10 to 0.8.11
- *(deps)* Bump rusqlite from 0.30.0 to 0.31.0
- *(deps)* Bump inquire from 0.6.2 to 0.7.0
- *(deps)* Bump walkdir from 2.4.0 to 2.5.0

### Release

- Version 0.5.1

## [0.5.0] - 2024-03-02

### 🚀 Features

- *(cli)* Set functions folder by config file
- *(cli)* Allow typescript functions
- *(server)* Add process.env
- *(sever)* Add asset cache segment
- Asset with path
- *(sever)* Add query app env option
- *(cli)* Add generate

### 🐛 Bug Fixes

- *(server)* Add missing sqlite params
- *(server)* Handle multipart/form-data
- *(cli)* Assets without valid utf-8
- *(server)* Re-set formdata

### 🚜 Refactor

- Add clippy fixes
- *(cli)* Remove unnecessary slash

### 📚 Documentation

- Fix folder structure
- Add query server app

### ⚙️ Miscellaneous Tasks

- Add check and clippy to dev commands
- Manage git tag

### Build

- *(deps)* Bump hyper from 1.0.1 to 1.1.0
- *(deps)* Bump anyhow from 1.0.75 to 1.0.76
- *(deps)* Bump tokio from 1.35.0 to 1.35.1
- *(deps)* Bump reqwest from 0.11.22 to 0.11.23
- *(deps)* Bump hyper-util from 0.1.1 to 0.1.2
- *(deps)* Bump openssl from 0.10.61 to 0.10.62
- *(deps)* Bump serde_bytes from 0.11.12 to 0.11.13
- *(deps)* Bump anyhow from 1.0.76 to 1.0.78
- *(deps)* Bump clap from 4.4.11 to 4.4.12
- *(deps)* Bump serde_json from 1.0.108 to 1.0.109
- *(deps)* Bump tabled from 0.14.0 to 0.15.0
- *(deps)* Bump serde from 1.0.193 to 1.0.195
- *(deps)* Bump serde_json from 1.0.109 to 1.0.111
- *(deps)* Bump serde_bytes from 0.11.13 to 0.11.14
- *(deps)* Bump clap from 4.4.12 to 4.4.13
- *(deps)* Bump anyhow from 1.0.78 to 1.0.79
- *(deps)* Bump clap from 4.4.13 to 4.4.14
- *(deps)* Bump rustyscript from 0.2.1 to 0.3.0
- *(deps)* Bump clap from 4.4.14 to 4.4.16
- Update cargo dist 0.5.0 to 0.7.2
- Update cargo dist 0.7.2 to 0.11.1

### Release

- Version 0.5.0

## [0.4.0] - 2023-12-14

### 🚀 Features

- Add deno prerequisites
- *(server)* Add asset builder
- *(server)* Add asset
- *(cli)* Add asset

### 🐛 Bug Fixes

- Version number

### 📚 Documentation

- Add query logo
- Add asset
- Update function

### Build

- *(deps)* Bump openssl from 0.10.59 to 0.10.60
- *(deps)* Bump serde from 1.0.192 to 1.0.193
- *(deps)* Bump url from 2.4.1 to 2.5.0
- Update cargo dist from 0.4.3 to 0.5.0
- *(deps)* Bump jsonwebtoken from 9.1.0 to 9.2.0
- *(deps)* Bump openssl from 0.10.60 to 0.10.61
- *(deps)* Bump tokio from 1.34.0 to 1.35.0
- *(deps)* Bump clap from 4.4.8 to 4.4.11
- *(deps)* Bump rustyline from 12.0.0 to 13.0.0

### Release

- Version 0.4.0

## [0.3.4] - 2023-11-22

### 🚀 Features

- *(ci)* Add dependabot

### 🚜 Refactor

- Add clippy fixes

### ⚙️ Miscellaneous Tasks

- Add tests

### Build

- *(deps)* Bump serde from 1.0.190 to 1.0.192
- *(deps)* Bump clap from 4.4.7 to 4.4.8
- *(deps)* Bump toml from 0.8.6 to 0.8.8
- *(deps)* Bump tokio from 1.33.0 to 1.34.0
- *(deps)* Bump rustyscript from 0.1.4 to 0.2.1
- *(deps)* Bump rusqlite from 0.29.0 to 0.30.0
- Add branches actions
- *(deps)* Bump uuid from 1.5.0 to 1.6.1
- *(deps)* Bump hyper from 0.14.27 to 1.0.1

### Release

- Version 0.3.4

## [0.3.3] - 2023-11-11

### 🚀 Features

- Enhance proxy testing
- Simplify proxy

### Release

- Version 0.3.3

## [0.3.2] - 2023-11-10

### 🚀 Features

- *(docs)* Update token secret example

### 🐛 Bug Fixes

- *(server)* Proxy query string

### Release

- Version 0.3.2

## [0.3.1] - 2023-11-10

### 🚀 Features

- *(docs)* Update proxy info

### 🐛 Bug Fixes

- *(server)* Proxy body utf8 error

### Release

- Version 0.3.1

## [0.3.0] - 2023-11-09

### 🚀 Features

- *(ci)* Remove dev profile
- *(cli)* Update version
- Remove example folder
- Add query and query server test commands
- *(server)* Rename config.sql to query_config.sql
- *(server)* Add proxy
- *(server)* Add a prefix to the entry points

### 🐛 Bug Fixes

- *(cli)* File not exist function test

### 🚜 Refactor

- *(server)* Move controllers

### Release

- Version 0.3.0

## [0.2.1] - 2023-11-04

### 🚀 Features

- *(ci)* Create query-server artifact
- *(ci)* Add Cargo.lock

### Release

- Version 0.2.1

## [0.2.0] - 2023-10-31

### 🚀 Features

- *(server)* Update dependencies
- Add function
- *(server)* Add is admin validation to migration

### 🐛 Bug Fixes

- *(server)* Remove statement panics
- Hurl tests

### 📚 Documentation

- Branch name format

### Release

- Version 0.2.0

## [0.1.0] - 2023-10-03

### ⚙️ Miscellaneous Tasks

- Create query

### 🚀 Features

- *(server)* Create manage branches
- *(docs)* Add migration endpoint to the table of content
- *(docs)* Add branch endpoint
- *(cli)* Create branch command
- *(docs)* Add branch command

### Release

- Version 0.1.0
