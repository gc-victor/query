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

There are two formats of output:

- **JSON** - Convert markdown files to JSON with a single command (Default)

```sh
pnpm query-docs --input ./src/docs --output ./dist/docs
```

It will generate a JSON file in the `./dist/docs` directory. With this JSON file, you can easily integrate your documentation into your application or website.

Or

- **HTML** - Convert markdown files to HTML with a single command

```sh
pnpm query-docs --input ./src/docs --output ./dist/docs --html
```

6. Your documentation is now ready in the `./dist/docs` directory!

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
    -o, --output <OUTPUT_DIR>         Directory to output HTML files
        --template <TEMPLATE_FILE>    HTML template file to use for generating pages (defaults to INPUT_DIR/template.html)
        --html                        Generate HTML files
        --search                      Generate search JSON file (Only works along with --html)
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

## Markdown to JSON

The markdowns will generate a set of JSON files representing the documentation structure.

The pages JSON schema is defined as follows:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["content", "description", "markdown", "metadata", "navigation", "path", "plain_text", "title"],
  "properties": {
    "content": {
      "type": "string",
      "description": "HTML content of the page"
    },
    "description": {
      "type": "string",
      "description": "Brief description of the page content"
    },
    "markdown": {
      "type": "string",
      "description": "Raw markdown content of the page"
    },
    "metadata": {
      "type": "object",
      "properties": {
          "description": {
            "type": "string",
            "description": "Generates an object containing metadata from the markdown content"
          }
      }
    },
    "navigation": {
      "type": "object",
      "required": ["current"],
      "properties": {
        "current": {
          "type": "object",
          "required": ["title", "url"],
          "properties": {
            "title": {
              "type": "string",
              "description": "Title of the current page"
            },
            "url": {
              "type": "string",
              "description": "URL of the current page",
              "pattern": "^\\.\\/.*\\.html$"
            }
          }
        },
        "next": {
          "type": ["object", "null"],
          "properties": {
            "title": {
              "type": "string",
              "description": "Title of the next page"
            },
            "url": {
              "type": "string",
              "description": "URL of the next page",
              "pattern": "^\\.\\/.*\\.html$"
            }
          },
          "required": ["title", "url"]
        },
        "previous": {
          "type": ["object", "null"],
          "description": "Information about the previous page, null if this is the first page"
        }
      }
    },
    "path": {
      "type": "string",
      "description": "Path to the current page",
      "pattern": "^\\.\\/.*\\.html$"
    },
    "plain_text": {
      "type": "string",
      "description": "Plain text version of the content"
    },
    "title": {
      "type": "string",
      "description": "Title of the page"
    }
  },
  "additionalProperties": false
}
```

Alongside of the pages, will be generated a `toc.json` file containing the table of contents structure.

The `toc.json` JSON schema is defined as follows:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["items"],
  "properties": {
    "items": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["name", "items"],
        "properties": {
          "name": {
            "type": "string",
          },
          "items": {
            "type": "array",
            "items": {
              "type": "object",
              "required": ["group", "title", "url", "level", "children"],
              "properties": {
                "group": {
                  "type": "string"
                },
                "title": {
                  "type": "string"
                },
                "url": {
                  "type": "string",
                  "pattern": "^\\.\\/.*\\.html$"
                },
                "level": {
                  "type": "integer",
                  "minimum": 1,
                  "maximum": 2
                },
                "children": {
                  "type": "array",
                  "items": {
                    "$ref": "#/properties/items/items/properties/items/items"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
```

You can find a fully working example on [examples/docs](https://github.com/gc-victor/query/tree/main/examples/docs) directory.

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
- [Query Docs Example - GitHub](https://github.com/gc-victor/query/tree/main/examples/docs)
- [MiniJinja Syntax](https://docs.rs/minijinja/latest/minijinja/syntax/index.html)
