# Functions

Query's Functions enables you to write serverless-like JavaScript functions that handle HTTP requests, interact with databases, and serve web content. These functions are individual endpoints that can process requests, manipulate data, and return responses - similar to Express.js routes or Netlify Functions, but with built-in SQLite database support.

## Basic Function Structure

Every Query function follows a simple but powerful pattern that will feel familiar to JavaScript developers who have worked with web APIs. The core building block is the `handleRequest` function, which receives incoming HTTP requests and returns responses.

```javascript
export async function handleRequest(req) {
  return new Response("Hello from Query!", {
    status: 200,
    headers: {
      "content-type": "text/plain",
    },
  });
}
```

## File-Based Routing

Query uses an intuitive file-based routing system similar to Next.js or SvelteKit. Your file structure automatically defines your API routes. There is no need for complex route configurations—just organize your files logically, and Query handles the rest.

```
src/
├── api/                           # API endpoints
│   ├── admin/                     # Admin API routes
│   │   ├── login/                 # Authentication
│   │   └── post/                  # Post management
│   │       ├── get.index.js       # GET "/api/admin/post"
│   │       ├── post.index.js      # POST "/api/admin/post"
│   │       ├── put.index.js       # PUT "/api/admin/post"
│   │       └── delete.index.js    # DELETE "/api/admin/post"
│   └── post/                      # Public post API
└── pages                          # Application pages
    ├── get.index.tsx              # Main
    ├── hot-reload                 # Hot reload
    │   ├── get.index.ts           # Hot reload server function
    │   └── hot-reload.tsx         # Hot reload client component
    ├── no-dynamic                 # No dynamic page
    │   └── get.index.tsx          # No dynamic page server function
    ├── [slug]                     # Dynamic page
    │   └── get.index.tsx          # Dynamic page server function
    └── styles.css                 # Global styles
```

### Dynamic Routes

Query's dynamic route syntax allows you to create flexible, parameter-based routes, handle variable paths, and create RESTful resources.

```javascript
// src/users/get.[slug].js | src/users/[slug]/get.index.js 
export async function handleRequest(req) {
  const segments = req.url.split("/");
  const userId = segments[segments.length - 1];

  const db = new Database("app.sql");
  const user = await db.query("SELECT * FROM users WHERE id = ?", [userId]);

  if (!user.length) {
    return new Response(JSON.stringify({ error: "User not found" }), {
      status: 404,
      headers: {
        "content-type": "application/json",
      },
    });
  }

  return new Response(JSON.stringify({ data: user[0] }), {
    status: 200,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

## JSX Server-Side

Query supports JSX syntax for building dynamic HTML responses, making it easy to generate complex HTML structures using components. It is **transpiled at build time** to standard JavaScript, so you can use JSX without any additional setup.

```javascript
// src/pages/get.index.js
import { Head, Body } from "@/pages/components";
import { App } from "@/pages/application";

export async function handleRequest(req) {
  return new Response(
    `<!DOCTYPE html>${
      <html lang="en">
        <Head>
          <title>Query</title>
        </Head>
        <Body>
          <App />
        </Body>
      </html>
    }`,
    {
      status: 200,
      headers: {
        "content-type": "text/html",
      },
    }
  );
}
```

To use JSX in your Query functions, make sure to add `"jsx" = "preserve"` to the `[esbuild]` section of your `Query.toml` file:

```toml
[esbuild]
"jsx" = "preserve"
```

If you are using TypeScript, you have to also add the following to your `tsconfig.json`:

```json
{
  "compilerOptions": {
    ...
    "jsx": "preserve",
    "jsxFactory": "jsx",
    "jsxFragmentFactory": "Fragment",
    ...
  }
}
```

### Inline JavaScript

You can add an inline JavaScript code by using the `script` tag in the JSX syntax. Here is an example:

```javascript
...
<script>
  ${`console.log("Hello from inline JavaScript!");`}
</script>
...
```

### String HTML

You can also use string HTML in the JSX syntax. Here is an example:

```javascript
...
<div>
  ${StringHTML(`<h1>Hello from string HTML!</h1>`)}
</div>
...
```

### Handling Different HTTP Methods

Query supports all standard HTTP methods, making it easy to build RESTful APIs or handle various types of requests. Here's how to work with different request types and their data.

#### GET with Query Parameters

Process URL parameters and search queries with the built-in URL API, making it easy to handle user inputs and search requests.

```javascript
// src/posts/search/get.index.js
export async function handleRequest(req) {
  const url = new URL(req.url);
  const query = url.searchParams.get("q");
  const db = new Database("app.sql");

  const results = await db.query("SELECT * FROM posts WHERE title LIKE ?", [`%${query}%`]);

  return new Response(JSON.stringify({ data: results }), {
    status: 200,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

#### POST with Form Data

Handle form submissions and file uploads using the standard FormData API, making it familiar for web developers.

```javascript
// src/posts/upload/post.index.js
export async function handleRequest(req) {
  const formData = await req.formData();
  const title = formData.get("title");
  const content = formData.get("content");

  const db = new Database("blog.sql");
  await db.query("INSERT INTO posts (title, content) VALUES (?, ?)", [title, content]);

  return new Response(JSON.stringify({ success: true }), {
    status: 201,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

## Working with Databases

Query provides a straightforward database interface through its `Database` class. Unlike traditional ORMs or database clients, Query's database operations are designed to be simple and SQLite-native while providing all the power you need for complex operations.

### Select Data

With Query's SQL-first approach, retrieving data from your database is straightforward. You can write natural SQL queries with parameter binding for safety and clarity.

```javascript
// src/users/get.index.js
import { Database } from "query:database";

export async function handleRequest(req) {
  const db = new Database("app.sql");
  const users = await db.query("SELECT * FROM users WHERE active = ?", [true]);

  return new Response(JSON.stringify({ data: users }), {
    status: 200,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

### Insert Data

Creating new records is just as simple, supporting positional and named parameters to match your preferred SQL style.

```javascript
// src/users/post.index.js
import { Database } from "query:database";

export async function handleRequest(req) {
  const { name, email } = await req.json();
  const db = new Database("app.sql");

  await db.query("INSERT INTO users (name, email) VALUES (:name, :email)", { ":name": name, ":email": email });

  return new Response(JSON.stringify({ success: true }), {
    status: 201,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

## Function Caching

Improve performance and reduce database load with Query's built-in caching system. Set the `Query-Cache-Control` header to cache responses for specified durations.

```javascript
// get.index.js
export async function handleRequest(req) {
  const db = new Database("app.sql");
  const data = await db.query("SELECT * FROM expensive_query");

  return new Response(JSON.stringify({ data }), {
    status: 200,
    headers: {
      "content-type": "application/json",
      "Query-Cache-Control": "max-age=3600000", // Cache for 1 hour
    },
  });
}
```

## Error Handling

Implement robust error handling using try-catch blocks and appropriate HTTP status codes. Query makes it easy to return meaningful error responses to clients.

```javascript
// post.index.js
export async function handleRequest(req) {
  try {
    const { token } = await req.json();

    if (!token) {
      return new Response(JSON.stringify({ error: "Unauthorized" }), {
        status: 401,
        headers: {
          "content-type": "application/json",
        },
      });
    }

    // Process protected route...
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: {
        "content-type": "application/json",
      },
    });
  }
}
```

## Best Practices

Follow these guidelines to write maintainable, efficient, and secure Query functions. These practices help ensure your application performs well and remains maintainable as it grows.

### Database Connections

Create database connections inside your functions to ensure proper resource management and avoid connection leaks:

```javascript
export async function handleRequest(req) {
  const db = new Database("app.sql");
  // Use db...
}
```

### Response Headers

Always set appropriate content-type headers to ensure correct client-side processing:

```javascript
return new Response(data, {
  headers: {
    "content-type": "application/json",
  },
});
```

### Error Responses

Return appropriate HTTP status codes and clear error messages for better client experience:

```javascript
if (!data) {
  return new Response(JSON.stringify({ error: "Not found" }), {
    status: 404,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

### Route Organization

Keep your function files organized by feature or resource for better maintainability:

```
functions/
├── api/
│   ├── auth/
│   ├── users/
│   └── posts/
└── pages/
    ├── admin/
    └── public/
```

### Cache Wisely

Use caching strategically for expensive operations or frequently accessed data:

```javascript
return new Response(data, {
    headers: {
        "Query-Cache-Control": "max-age=3600000", // 1 hour
    },
});
```
