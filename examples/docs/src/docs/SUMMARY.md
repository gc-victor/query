# Summary

## User Guide

- [Introduction](./introduction.md) Discover Query: a unified solution for web development that simplifies databases, JavaScript runtime, backend frameworks, caching, storage, scaffolding, and administration.
- [Quick Start](./quick-start.md) Get your Query project running in minutes with the quick start guide. Create a new application from templates and launch a local development server with minimal setup.
- [Install](./install.md) Get started with Query by installing the CLI and Server components. Choose from npm/pnpm packages, the installer script, or project templates for quick setup.
- [Configuration](./configuration.md) Configure Query projects with Query.toml and environment variables. Customize server settings, project structure, build parameters, and task automation for your applications.

## Server

- [Local Server](./server/local-server.md) Set up and run Query Server locally for development. Configure environment variables and project settings to create a development environment with hot reloading.
- [Server On Fly.io](./server/run-server-on-fly.md) Deploy Query Server to Fly.io with global database replication. Configure LiteFS for distributed SQLite databases with high availability across multiple regions.
- [Server App](./server/server-app.md) Configure Query Server as a standalone application server. Remove function prefixes from URLs to create cleaner routes for pages and API endpoints.
- [Server Proxy](./server/server-proxy.md) Set up Query Server as a proxy to your application. Access SQLite databases directly while benefiting from Query's authentication, CLI, and management features.
- [JavaScript Runtime](./server/runtime.md) Explore Query's JavaScript runtime built on QuickJS. Discover supported APIs, Web Platform features, and compatibility with the WinterCG specification.
- [Plugin System](./server/plugin-system.md) Extend Query functionality with WebAssembly plugins written in multiple languages. Learn about the plugin architecture and how to create custom plugins.

## Modules

- [Function](./modules/function.md) Build serverless functions with Query's runtime environment. Handle HTTP requests, connect to databases, and deliver dynamic content with file-based routing.
- [Database](./modules/database.md) Interface with SQLite databases using Query's Database module. Execute SQL queries with parameter binding, handle transactions, and manage database connections in your functions.
- [Email](./modules/email.md) Send emails with attachments and inline content using Query's email module. Configure SMTP servers or use the built-in service with simple JavaScript API calls.
- [Plugin](./modules/plugin.md) Extend Query with WebAssembly plugins using the plugin module. Execute functions from WASM files with configurable memory, permissions, and timeouts.

## Command Line

- [Asset](./cli/asset.md) Learn to upload and manage static assets in Query Server with the asset CLI command. Store files with optimal cache configurations and serve them via dedicated endpoints.
- [Branch](./cli/branch.md) Manage database branches efficiently with Query's CLI branch commands. Create copies of databases for development and testing while preserving production data.
- [Create](./cli/create.md) Quickly bootstrap new Query projects with the create command. Choose from predefined templates or use custom GitHub repositories to start your database-driven application.
- [Deploy](./cli/deploy.md) Deploy Query projects to production servers with configurable options. Use environment variables or interactive prompts to securely send your project to remote servers.
- [Dev](./cli/dev.md) Set up a local development environment for Query applications with live reloading. Watch files in src, dist, and public folders with automatic server updates.
- [Function](./cli/function.md) Create and manage serverless JavaScript functions in Query Server. Handle HTTP requests, connect to databases, and implement route-based functionality with file-based routing.
  - [Plugin](./cli/plugin.md) Extend Query's functionality with WASM plugins. Install, update, and deploy plugins from GitHub repositories to add custom functionality to your Query applications.
- [Generate](./cli/generate.md) Accelerate development with Query's code generation tools. Create database schemas and corresponding code files from simple commands that define tables and columns.
- [Migration](./cli/migration.md) Manage database schema changes with Query's migration system. Create versioned migration files to evolve your database structure while maintaining data integrity.
- [Settings](./cli/settings.md) Configure Query CLI authentication and connection settings. Securely store server URLs, credentials, and tokens for seamless interaction with Query Server.
- [Shell](./cli/shell.md) Access and manage remote SQLite databases with Query's interactive shell. Execute SQL commands directly against server databases with command history support.
- [Task](./cli/task.md) Define and execute custom commands in Query projects. Configure reusable tasks in Query.toml for development, building, and deployment automation.
- [Test](./cli/test.md) Write and run JavaScript/TypeScript tests with Query's built-in test runner. Enjoy Jest-like syntax with assertions, lifecycle hooks, and spying capabilities.
- [Token](./cli/token.md) Manage authentication tokens for Query Server access. Create, list, update, and delete non-user-specific tokens with customizable permissions and expiration dates.
- [User](./cli/user.md) Manage user accounts in Query Server through the CLI. Create, update, delete users, and modify permissions with administrative access controls.
  - [User Token](./cli/user-token.md) Administer user-specific authentication tokens in Query Server. Create, list, delete, and update tokens with customizable access permissions for individual users.

## API

- [Query](./api/query.md) Discover Query's data retrieval API with optimized latency through LiteFS proxy. Learn to execute SELECT queries and write operations with parameter binding for SQLite databases.
- [User](./api/user.md) Administer users in Query Server with comprehensive REST endpoints for creating, updating, and deleting user accounts with customizable permissions and authentication.
- [User Token](./api/user-token.md) Manage user authentication with Query's user token API. Create, update, and delete user-specific tokens with customizable permissions and retrieve token values.
- [Token](./api/token.md) Master server authentication with Query's token management API. Create, list, update, and delete access tokens with customizable permissions and expiration dates.
- [Migration](./api/migration.md) Understand how to execute database migrations in Query Server using the migration API endpoint with authenticated POST requests and required parameters.
- [Branch](./api/branch.md) Learn how to manage database branches in Query Server with REST endpoints. Create, list, and delete branches using the branch API with proper authentication and parameters.
