# Query Application

"Query Application" is a basic blog platform built with Query, demonstrating modern web development practices with features
like server-side rendering, admin dashboard, post management, and real-time hot reloading. It provides a foundation
for building applications with Query. Please keep in mind that its purpose is to be a bootstrap tool; it doesn't give a blog post editor;
as it is, it would work alongside a headless CMS.

## Features

- **Server-Side Rendering**: Fast, SEO-friendly page rendering
- **Admin Dashboard**: Secure admin interface for content management
- **Post Management**: Create, read, update, and delete blog posts
- **Authentication**: Secure admin login with session management
- **Image Handling**: Upload and manage post images
- **Hot Reloading**: Real-time development updates
- **SQLite Database**: Lightweight, serverless database integration
- **Responsive Design**: Mobile-friendly interface using Tailwind CSS

## Getting Started

1. Create a new project:

```sh
# With pnpm
pnpm dlx @qery/query create

# With npm
npx @qery/query create
```

> [!IMPORTANT]  
> Select the `application` project and follow the steps to create a new project.

2. Start the development server:

```sh
# With pnpm
pnpm dev

# With npm
npm run dev
```

3. Open your browser and navigate to:

- Blog: `http://localhost:3000`
- Admin: `http://localhost:3000/admin`

## Project Structure

```
migrations/                    # Database migrations
├── admin_user_session.sql/    # Admin session management
└── post.sql/                  # Blog post schema and initial data

src/
├── api/                       # API endpoints
│   ├── admin/                 # Admin API routes
│   │   ├── login/             # Authentication
│   │   └── post/              # Post management
│   └── post/                  # Public post API
├── config/                    # Configuration files
├── lib/                       # Shared utilities
└── pages/                     # Application pages
    ├── admin/                 # Admin interface
    ├── components/            # Shared components
    ├── hot-reload/            # Development tools
    ├── layouts/               # Page layouts
    └── post/                  # Blog post pages
```

## Key Features Implementation

- **Authentication**: Uses secure session management with SQLite storage
- **Post Management**: Full CRUD operations with image upload support
- **Real-time Updates**: Hot reloading during development
- **Validation**: Form validation using Valibot schema
- **SEO**: Server-side rendered pages with proper meta tags
- **Asset Management**: Efficient handling of images and web fonts

## References

- [Query Documentation](https://qery.io)
- [Query GitHub Repository](https://github.com/gc-victor/query)
- [Blog Application Example](https://github.com/gc-victor/query/tree/mainexamples/application)
