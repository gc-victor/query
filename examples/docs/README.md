# Query Docs

"Query Docs" is a lightning-fast markdown documentation generator that transforms your markdown files into beautiful, navigable documentation websites.

## Getting Started

1. Install the package:

```sh
# With pnpm
pnpm add @qery/docs
```

Or

```sh
# With npm
npm install @qery/docs
```

2. Create a directory for your documentation:

```sh
mkdir src/docs
```

3. Create a in the `src/docs` SUMMARY.md file that defines your documentation structure:

```markdown
# Summary

## Getting Started

- [Introduction](introduction.md) Description of the introduction
- [Installation](installation.md) Description of the installation process

## User Guide

- [Basic Usage](usage/basic.md) Description of basic usage
- [Advanced Features](usage/advanced.md) Description of advanced features
```

4. Create the corresponding markdown files mentioned in your SUMMARY.md

5. Generate your documentation:

```sh
pnpm query-docs --input ./src/docs --output ./dist/docs
```

6. Your documentation is now ready in the `./dist/docs` directory!

> [!TIP]
> Create a custom template.html in your docs directory to fully customize the look and feel of your documentation.

## Key Features

- **Effortless Documentation** - Convert markdown files to HTML with a single command
- **Smart Navigation** - Automatically generates previous/next links between pages
- **Hierarchical TOC** - Creates a structured table of contents from your SUMMARY.md
- **Full-Text Search** - Built-in search functionality with JSON index
- **Customizable Templates** - Use your own HTML templates for full control over styling
- **Fast & Lightweight** - Built in Rust for exceptional performance

## Project Structure

A typical Query Docs project includes:

```sh
docs/
├── SUMMARY.md          # Documentation structure
├── template.html       # Custom HTML template (optional)
├── introduction.md     # Content pages
├── installation.md
└── usage/
    ├── basic.md
    └── advanced.md
```

## Command Line Options

```sh
USAGE:
    pnpm query-docs [OPTIONS] --input <INPUT_DIR> --output <OUTPUT_DIR>

OPTIONS:
    -i, --input <INPUT_DIR>           Directory containing markdown files
    -o, --output <OUTPUT_DIR>

Directory to output HTML files
    -t, --template <TEMPLATE_FILE>    HTML template file to use (default: INPUT_DIR/template.html)
        --no-search                   Disable search functionality
    -h, --help                        Print help information
    -V, --version                     Print version information
```

## SUMMARY.md Format

The SUMMARY.md file defines the structure of your documentation using a simple format:

```markdown
# Summary

## Section 1

- [Page Title](path/to/page.md) Optional description
  - [Nested Page](path/to/nested.md) Optional description

## Section 2

- [Another Page](another-page.md) Optional description
```

- Use `##` headings to create sections
- Use bullet points (`-`) followed by Markdown links to define pages
- Indented bullet points create nested pages in the hierarchy

## Template Customization

Your HTML template has access to these variables:

- `title`: The page title
- `description`: The page description
- `content`: The HTML content
- `navigation`: Previous/next page navigation
    - `previous`: Previous page information
    - `next`: Next page information
- `toc`: Table of contents structure
- `search_enabled`: Boolean for search feature

Example template snippet:

```html
<main>
  <article>{{ content }}</article>

  <nav class="pagination">
    {% if navigation.previous %}
    <a href="{{ navigation.previous.url }}">← {{ navigation.previous.title }}</a>
    {% endif %} {% if navigation.next %}
    <a href="{{ navigation.next.url }}">{{ navigation.next.title }} →</a>
    {% endif %}
  </nav>
</main>
```

## References

- [Query Website](https://qery.io)
- [Query - GitHub](https://github.com/gc-victor/query)
- [MiniJinja Syntax](https://docs.rs/minijinja/latest/minijinja/syntax/index.html)

## Examples

### Basic Documentation Site

```bash
# Project structure
docs/
├── SUMMARY.md
├── template.html
├── introduction.md
├── getting-started.md
└── api-reference.md

# Generate documentation
pnpm query-docs --input ./src/docs --output ./dist/docs
```

### Multi-section Documentation

```markdown
# Summary

## Introduction

- [Overview](index.md)
- [Getting Started](getting-started.md)

## API Reference

- [Authentication](api/auth.md)
- [Endpoints](api/endpoints.md)
  - [Users API](api/endpoints/users.md)
  - [Products API](api/endpoints/products.md)
```
