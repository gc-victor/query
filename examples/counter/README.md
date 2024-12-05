# Query Counter

"Query Counter" is a project to start using Query on your local device.

## Getting Started

1. Create a new project:

```sh
# With pnpm
pnpm dlx @qery/query create

# With npm
npx @qery/query create
```

> [!IMPORTANT]
> Select the `counter` project and follow the steps to create a new project.

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

The counter project includes:

```
migrations                      # Database migrations
└── counter.sql                 # Counter database
    └── 001_counter.sql         # Create counter table
src                             # Application source code
├── api                         # API endpoints
│   └── counter                 # Counter API
│       └── put.index.ts        # Update counter
└── pages                       # Application pages
    ├── counter.island.tsx      # Counter client component
    ├── counter.tsx             # Counter component (server/client)
    ├── get.index.tsx           # Main counter page
    ├── hot-reload              # Hot reload
    │   ├── get.index.ts        # Hot reload server function
    │   └── hot-reload.tsx      # Hot reload client component
    ├── lib                     # Library functions
    │   ├── get-asset-path.ts   # Get asset path from the database
    │   └── render.ts           # Server-side page rendering
    └── styles.css              # Global styles
```

## Main Page Structure

The `src/pages/get.index.tsx` file serves as the main entry point for the counter application. It demonstrates several key Query features:

### File-Based Routing

The file structure follows Query's routing convention:

- `pages/get.index.tsx` -> Handles GET requests at root ('/')
- `api/counter/put.index.ts` -> Handles PUT requests at '/api/counter'

### Counter Page Structure

```tsx
export async function handleRequest(req: Request) {
  const db = new Database("counter.sql");
  const [counter] = db.query("SELECT value FROM counter WHERE id = 1");
  const initialValue = counter.value;

  return new Response(
    render(
      <>
        {/* Head section with meta, styles, and scripts */}
        <body>
          <div className="flex flex-col items-center p-8 justify-center h-screen">
            <counter-island>
              <Counter count={initialValue} />
            </counter-island>
          </div>
        </body>
      </>,
    ),
    {
      headers: {
        "Content-Type": "text/html; charset=utf-8",
      },
    },
  );
}
```

Key features:

- Fetches initial counter value from SQLite database
- Server-side renders the counter interface
- Integrates client-side interactivity via Web Components
- Uses Tailwind CSS for styling

### Counter API Structure

The `src/api/counter/put.index.ts` handles counter state updates:

```ts
export async function handleRequest(req: Request) {
  const { value } = await req.json();

  const db = new Database("counter.sql");
  db.query("UPDATE counter SET value = ?, updated_at = CURRENT_TIMESTAMP WHERE id = 1", [value]);

  return new Response(JSON.stringify({ value }), {
    status: 200,
    headers: {
      "content-type": "application/json",
    },
  });
}
```

The API endpoint demonstrates:

- Request body parsing with JSON
- Database updates with parameterized queries
- Proper error handling
- JSON response formatting

### Database Integration

The application uses SQLite through Query's Database class:

```ts
const db = new Database("counter.sql");
```

Features:

- Persistent counter state storage
- Parameterized queries for safety
- Automatic timestamp tracking
- Built-in SQLite support

### Hot Reload Integration

The page includes the `HotReload` component for development:

```tsx
<HotReload href={url.href} />
```

Enabling instant updates during development without full page refreshes.

## Features

- JSX server-side rendering
- Hot module replacement
- SQLite database integration
- API endpoints
- Web Components for interactivity
- Tailwind CSS styling

## References

- [Query Website](https://qery.io)
- [Query - GitHub](https://github.com/gc-victor/query)
- [Query Minimal - GitHub](https://github.com/gc-victor/query/tree/mainexamples/minimal)
