# Function

A function is a JavaScript function that is executed in the Query Server and it has access to the databases.

The function should be in the format of:

```js
export async function handleRequest(req) {
    return new Response("This is the body!", {
      status: 200,
      headers: {
          "content-type": "text/plain",
      },
  });
}
```

The function has to export a function called `handleRequest` that receives a [Request](https://developer.mozilla.org/en-US/Web/API/Request) and returns a [Response](https://developer.mozilla.org/en-US/Web/API/Response).

To use a database you have to create a connection to the database:

```js
const db = new Database("example.sql");
```

You can use it as a global variable or `import` it.

```js
import { Database } from "query:database";
```

The `Database` constructor receives the name of the database. If the database is found, it will create a connection to the database; if not, it will create it first. It exposes the method `query` to read data from the database and write data in the database. The method `query` can have params, and those params are bound to the parameters based on the order of the array or an object with the format of `:AAA`, `$AAA`, or `@AAA` that serve as placeholders for values that are bound to the parameters at a later time.

As Query uses [LiteFS proxy](https://fly.io/litefs/config/#http-proxy-server), you have to remember to use `GET` to read data and `DELETE|POST|PUT|PATCH` to write data.

## Handle Request Example

```js
// get.index.js
export async function handleRequest(req) {
    const db = new Database("example.sql");

    const result = await db.query("SELECT * FROM example WHERE id = ?", [1]);

    return new Response(JSON.stringify({data: result}), {
      status: 200,
      headers: {
          "content-type": "application/json",
      },
  });
}
```

Query CLI provides an API to resolving routes against file-system paths and using the file names. To use functions it is required to follow the next structure:

## Folder Structure Example

```sh
functions
├── get.index.js // GET "/"
├── post.index.js // POST "/"
├── example
    ├── get.index.js // GET "/example"
    └── get.[slug].js // GET "/example/:slug"
├── [slug]
    └── get.index.js  // GET "/:slug"
...
```

By default the folder to contain the functions has to be called `functions`. You can use another one by pointing to it, but we will explain it with more detail below.

It is important to note that the method used in a file is determined by the prefix `(delete|get|patch|post|put).*`, while the remaining part of the file name defines the final segment of the route. For instance, if the file name ends with `index`, it will be the root of the route, and if it is `[slug]`, it will be a route with a slug. The slug is a placeholder for a value used in the route.

To define the different segments of the route, you must use the folder structure. For example, if you want to use the path `/example/:slug`, you have to create a folder called `example` and inside it a file called `get.[slug].js`. If you want to use the route `/:slug`, you have to create a folder called `[slug]` and inside of it a file called `get.index.js`. If you want to use the route `/`, you must create a file called `get.index.js`.

## Query Cache Control

The Query Server has a feature that helps avoid compiling functions that have not been modified, which in turn speeds up each response. This feature is managed using the `Query-Cache-Control` header and specifying the `max-age`, in milliseconds, in the header response of the `handleRequest` function. The function response is stored in the `cache_function` table of the `query_cache_function.sql` database. If needed, the cache can be purged by either deleting the row related to a path or by deleting the entire cache from the `cache_function` table.

```js
// get.index.js
export async function handleRequest(req) {
    const db = new Database("example.sql");

    const result = await db.query("SELECT * FROM example WHERE id = ?", [1]);

    return new Response(JSON.stringify({data: result}), {
      status: 200,
      headers: {
          "Content-Type": "application/json",
          "Query-Cache-Control": "max-age=3600000", // 1 hour
      },
  });
}
```

To purge the query cache control, you can use the following query command:

```sh
query purge
```

## Usage

Query uses under the hood [esbuild](https://esbuild.github.io) to bundle the functions. So, first you have to install esbuild:

```sh
npm install esbuild
```

Or

```sh
pnpm install esbuild
```

To use the functions you have to run the following command:

```sh
query function <PATH>
```

The path is optional. If you don't provide it, it will use the default path `functions`. You can use the path to point to another folder or a function file.

## Example

```sh
query function
```

It will deploy all the functions to the Query Server. A simple cache is implemented to avoid deploying functions that have not changed.

```sh
query function another-functions-folder
```

It will deploy all the functions in the `another-functions-folder` folder to the Query Server.

```sh
query function functions/get.index.js
```

It will deploy the `get.index.js` function to the Query Server.

```sh
query function functions/get.index.js --delete
```

It will delete the `get.index.js` function from the Query Server.
