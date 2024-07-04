# Changelog

All notable changes to this project will be documented in this file.

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
