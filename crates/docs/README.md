# Query Docs

A lightning-fast, feature-rich markdown documentation generator. Transform your markdown files into beautiful, navigable documentation websites.

## Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage](#-usage)
  - [Command Line Options](#command-line-options)
  - [SUMMARY.md Format](#summarymd-format)
  - [Template Customization](#template-customization)
- [Examples](#-examples)
- [Project Structure](#-project-structure)
- [Contributing](#-contributing)
- [Testing](#-testing)
- [License](#-license)
- [FAQ](#-faq)

## Features

- **Effortless Documentation** - Convert markdown files to HTML with a single command
- **Smart Navigation** - Automatically generates previous/next links between pages
- **Hierarchical TOC** - Creates a structured table of contents from your SUMMARY.md
- **Full-Text Search** - Built-in search functionality with JSON index
- **Customizable Templates** - Use your own HTML templates for full control over styling
- **Fast & Lightweight** - Built in Rust for exceptional performance

## Installation

### For JavaScript

```bash
pnpm add @qery/docs
```

OR

```bash
npm install @qery/docs
```

### From Source

```bash
git clone https://github.com/gc-victor/query.git
cargo build --package query-docs --release
```

The compiled binary will be available in `./target/release/query-docs`.

## Quick Start

1. Create a directory for your documentation:

```bash
mkdir docs
cd docs
```

2. Create a SUMMARY.md file that defines your documentation structure:

```markdown
# Summary

## Getting Started

- [Introduction](introduction.md) Description of the introduction
- [Installation](installation.md) Description of the installation process

## User Guide

- [Basic Usage](usage/basic.md) Description of basic usage
- [Advanced Features](usage/advanced.md) Description of advanced features
```

3. Create the corresponding markdown files mentioned in your SUMMARY.md

4. Create a template.html file in the same directory or specify a custom one

5. Generate your documentation:

```bash
query-docs --input ./docs --output ./build
```

6. Your documentation is now ready in the `./build` directory!

## Usage

### Command Line Options

```
USAGE:
    query-docs [OPTIONS] --input <INPUT_DIR> --output <OUTPUT_DIR>

OPTIONS:
    -i, --input <INPUT_DIR>           Directory containing markdown files
    -o, --output <OUTPUT_DIR>         Directory to output HTML files
    -t, --template <TEMPLATE_FILE>    HTML template file to use by default INPUT_DIR/template.html
        --no-search                   Disable search functionality
    -h, --help                        Print help information
    -V, --version                     Print version information
```

### SUMMARY.md Format

The SUMMARY.md file defines the structure of your documentation. It follows a simple format:

```markdown
# Summary

## Section 1

- [Page Title](path/to/page.md)
  - [Nested Page](path/to/nested.md)

## Section 2

- [Another Page](another-page.md)
```

- Use `##` headings to create sections in your table of contents
- Use bullet points (`-`) followed by Markdown links to define pages
- Indented bullet points create nested pages in the hierarchy

### Template Customization

`@qery/docs` uses the [MiniJinja](https://crates.io/crates/minijinja) template engine. Your template has access to the following variables:

- `title`: The title of the current page
- `description`: The description of the current page
- `content`: The HTML content of the current page
- `navigation`: Navigation object with:
  - `previous`: Previous page (title and URL) or null
  - `next`: Next page (title and URL) or null
  - `current`: Current page (title and URL)
- `toc`: Table of contents object
- `search_enabled`: Boolean indicating if search is enabled

For more information about MiniJinja syntax, see the [documentation](https://docs.rs/minijinja/latest/minijinja/syntax/index.html).

#### Example Template

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }}</title>
    <meta name="description" content="{{ description }}">
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <nav class="toc">
        <h1>Table of Contents</h1>
        {% for group in toc.items %}
            <h2>{{ group.name }}</h2>
            <ul>
            {% for item in group.items %}
                <li><a href="{{ item.url }}">{{ item.title }}</a></li>
            {% endfor %}
            </ul>
        {% endfor %}
    </nav>

    <main>
        <article>
            {{ content }}
        </article>
    </main>

    <nav class="pagination">
        {% if navigation.previous %}
            <a href="{{ navigation.previous.url }}" class="prev">← {{ navigation.previous.title }}</a>
        {% endif %}
        
        {% if navigation.next %}
            <a href="{{ navigation.next.url }}" class="next">{{ navigation.next.title }} →</a>
        {% endif %}
    </nav>

    {% if search_enabled %}
    <script src="search.js"></script>
    {% endif %}
</body>
</html>
```

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
query-docs --input ./docs --output ./site
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

## Tutorials
- [Quick Start](tutorials/quickstart.md)
- [Advanced Usage](tutorials/advanced.md)
```

### Using Custom Template

```bash
# Generate with custom root template
query-docs --input ./docs --output ./site --template ./custom-template.html
```

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

Please make sure your code passes all tests and follows the project's coding style.

## Testing

@qery/docs uses Rust's built-in testing framework. Run the tests with:

```bash
cargo test
```

The test suite includes unit tests and integration tests to ensure all features work correctly.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## FAQ

### How is @qery/docs different from mdBook?

While mdBook is a fantastic tool, @qery/docs focuses on providing a simpler, more lightweight solution with enhanced navigation and customizable templates. It's designed to be easy to use while still offering powerful features like hierarchical TOC and built-in search.

### Can I use custom CSS with @qery/docs?

Yes! You can include any CSS in your HTML template. Additionally, you can reference external CSS files that you can place in your output directory.

### Does @qery/docs support syntax highlighting?

Yes, code blocks in your markdown are properly preserved and can be syntax-highlighted using any JavaScript library of your choice (like Prism.js or highlight.js) by including it in your template.

### Can I deploy the generated site to GitHub Pages?

Absolutely! The generated HTML is static and can be deployed to any static site hosting service like GitHub Pages, Netlify, or Vercel.
