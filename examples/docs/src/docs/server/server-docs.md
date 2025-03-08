# Query Docs Server

Query Docs is a comprehensive documentation system that integrates seamlessly with Query Server, allowing you to build beautiful, interactive documentation sites. This guide will walk you through how to set up and use Query Docs in your Query Server project.

## Overview

Query Docs provides:

- A responsive documentation site with dark/light mode support
- Full-text search functionality
- Navigation between documentation pages
- Code syntax highlighting
- Support for error pages (404, 500)
- Table of contents for easy navigation

## Quick Start

1. Create a new project:

```sh
# With pnpm
pnpm dlx @qery/query create

# With npm
npx @qery/query create
```

<div class="markdown-alert">

> [!IMPORTANT]
>
> ### IMPORTANT!{.markdown-alert-title .font-cal}
>
> Select the `docs` project and follow the steps to create a new project.

</div>

You will have a fully functional documentation website with a default theme and layout.

2. Start the development server::

```bash
pnpm dev
```

## Project Structure

A typical Query Docs implementation consists of the following components:

```sh
src/
└── docs/                           # Root directory for documentation markdown files
└── api/
│   └── search/
│       └── get.index.tsx           # Search API endpoint
└── pages/
    └── docs/
        ├── [slug]/
        │   └── get.index.tsx       # Handler for top-level doc pages
        │   └── [slug]/
        │       └── get.index.tsx   # Handler for nested doc pages
        ├── components/             # Reusable UI components
        ├── island/                 # Client-side interactive components
        ├── 404.tsx                 # Not found page
        ├── 500.tsx                 # Error page
        ├── docs.tsx                # Main docs page template
        ├── get.index.tsx           # Root docs handler (redirects to intro)
        ├── handle-error.ts         # Error handling logic
        ├── styles.css              # Documentation styles
        └── types.ts                # TypeScript interfaces
```

## Key Components Explained

We are going to explain the key components of the Query Docs created.

### Main Documentation Page

The `docs.tsx` file defines the main template for documentation pages:

```tsx
export function DocsPage({ page, url, toc }: {page: DocumentationPage, url: URL, toc: Toc}): JSX.Element {
    return <html lang="en">
        <head>
            {/* Meta tags and stylesheets */}
        </head>
        <body class="flex min-h-full bg-white antialiased dark:bg-slate-900">
            <DocumentTemplate content={page.content} navigation={page.navigation as Navigation} toc={toc} />
            <Icons />
            <Scripts />
        </body>
    </html>;
}
```

### Route Handlers

Each route handler retrieves the right documentation page and table of contents:

```tsx
// For top-level pages: [slug]/get.index.tsx
async function handleDocsRequest(req: Request) {
    const url = new URL(req.url);
    const slug = url.pathname.split("/").pop();

    const toc = getAssetData<Toc>("dist/docs/toc.json");
    const page = getAssetData<DocumentationPage>(`dist/docs/${slug?.replace(/\.html$/, "")}.json`);

    return response(<DocsPage page={page} url={url} toc={toc} />);
}
```

### Error Handling

Custom error pages are implemented in `404.tsx` and `500.tsx`, with error handling logic in `handle-error.ts`:

```tsx
export function handleError(): ((req: Request, e: Error) => Response | undefined) | undefined {
    return (req, error) => {
        const url = new URL(req.url);
        const toc = getAssetData<Toc>("dist/docs/toc.json");

        if (error instanceof NotFoundError) {
            return NotFoundResponse({ origin: url.origin, toc });
        }

        return InternalServerErrorResponse({ origin: url.origin, toc });
    };
}
```

### Interactive Features

Query Docs includes several client-side features implemented as islands:

1. **Dark Mode Toggle**: Allows users to switch between light and dark themes.

2. **Search Functionality**: Enables full-text search across documentation pages.

3. **Code Syntax Highlighting**: Automatically highlights code blocks based on language.

## Documentation Format

Your documentation pages should be structured as JSON files with the following format:

```json
{
  "title": "Introduction",
  "description": "Introduction to Query",
  "content": "<div class='markdown-content'>...</div>",
  "markdown": "# Introduction\n...",
  "plain_text": "Introduction to Query...",
  "path": "/docs/introduction.html",
  "navigation": {
    "current": {
      "title": "Introduction",
      "url": "/docs/introduction.html"
    },
    "next": {
      "title": "Getting Started",
      "url": "/docs/getting-started.html"
    },
    "previous": null
  }
}
```

## Table of Contents Structure

The table of contents (`toc.json`) should follow this structure:

```json
{
  "items": [
    {
      "name": "Getting Started",
      "items": [
        {
          "title": "Introduction",
          "url": "/docs/introduction.html",
          "level": 1
        },
        {
          "title": "Installation",
          "url": "/docs/installation.html",
          "level": 1
        }
      ]
    },
    {
      "name": "Core Concepts",
      "items": [
        // ...
      ]
    }
  ]
}
```

## Styling Your Documentation

Query Docs uses Tailwind CSS for styling. You can customize the appearance by modifying the `styles.css` file. It includes:

- Typography styles for documentation content
- Dark mode support
- Alert boxes styling
- Custom scrollbars
- Responsive layout for mobile and desktop

## Extending Functionality

### Adding New Components

To add new components to your documentation:

1. Create a new component file in the `components` directory
2. Import and use it in your `DocumentTemplate` or other components

### Custom Interactive Features

For client-side interactivity:

1. Create a new `.island.ts` file in the `island` directory
2. Import and use the island script in your main template
