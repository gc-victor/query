# Query Minimal

"Query Minimal" is a project to start using Query on your local device.

## Getting Started

1. Create a new project:

```sh
# With pnpm
pnpm dlx @qery/query create

# With npm
npx @qery/query create
```

> [!IMPORTANT]
> Select the `minimal` project and follow the steps to create a new project.

2. Start the development server:

```sh
# With pnpm
pnpm dev

# With npm
npm run dev
```

3. Open your browser and navigate to:

```
http://localhost:3000
```

## Project Structure

The minimal project includes:

```
src
└── pages                  # Application pages
    ├── get.index.tsx      # Main
    ├── hot-reload         # Hot reload
    │   ├── get.index.ts   # Hot reload server function
    │   └── hot-reload.tsx # Hot reload client component
    ├── no-dynamic         # No dynamic page
    │   └── get.index.tsx  # No dynamic page server function
    ├── render.ts          # Server-side page rendering
    ├── [slug]             # Dynamic page
    │   └── get.index.tsx  # Dynamic page server function
    └── styles.css         # Global styles
```

## Main Page Structure

The `src/pages/get.index.tsx` file serves as the main entry point for the application. It demonstrates several key Query features:

### File-Based Routing

The file structure follows Query's routing convention:

- `pages/get.index.tsx` -> Handles GET requests at root ('/')
- `pages/[slug]/get.index.tsx` -> Handles dynamic routing GET requests
- `pages/no-dynamic/get.index.tsx` -> Handles GET requests at '/no-dynamic'

For example:

- `/dynamic` -> `[slug]` value is "dynamic"
- `/test` -> `[slug]` value is "test"

### Function Structure

```tsx
export async function handleRequest(req: Request) {
  // ... implementation
}
```

The `handleRequest` function is the core building block that:

- Receives incoming HTTP requests
- Processes the request
- Returns a Response object with HTML content

### Database Integration

```tsx
const db = new Database("query_asset.sql");
const result = db.query("SELECT name_hashed FROM asset WHERE name = ?", ["dist/styles.css"]);
```

Demonstrates Query's built-in SQLite database support with:

- Simple database connections
- SQL query execution with parameter binding
- Asset management for styles and resources

### Response Handling

```tsx
return new Response(render(/* JSX Content */), {
  status: 200,
  headers: {
    "Content-Type": "text/html; charset=utf-8",
  },
});
```

Shows proper response formatting with:

- Status codes
- Content-Type headers
- Server-side rendered JSX content

### Hot Reload Integration

The page includes the `HotReload` component for development:

```tsx
<HotReload href={url.href} />
```

Enabling instant updates during development without full page refreshes.

## Features

- JSX server-side rendering
- Hot module replacement
- Dynamic pages
- Tailwind CSS styling

## References

- [Query Website](https://qery.io)
- [Query - GitHub](https://github.com/gc-victor/query)
- [Query Minimal - GitHub](https://github.com/gc-victor/query/tree/mainexamples/minimal)
