# Query Docs

A lightning-fast, feature-rich markdown documentation generator. Transform your markdown files into beautiful, navigable documentation websites.

## Features

- **Effortless Documentation** - Convert markdown files to HTML with a single command
- **Smart Navigation** - Automatically generates previous/next links between pages
- **Hierarchical TOC** - Creates a structured table of contents from your SUMMARY.md
- **Full-Text Search** - Built-in search functionality with JSON index
- **Customizable Templates** - Use your own HTML templates for full control over styling
- **Fast & Lightweight** - Built in Rust for exceptional performance

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

### SUMMARY.md Format

The SUMMARY.md file defines the structure of your documentation. It follows a simple format:

```markdown
# Summary

## Introduction

- [Overview](index.md) Description ...
- [Getting Started](getting-started.md) Description ...

## API Reference

- [Authentication](api/auth.md) Description ...
- [Endpoints](api/endpoints.md) Description ...
  - [Users API](api/endpoints/users.md) Description ...
  - [Products API](api/endpoints/products.md) Description ...

## Tutorials

- [Quick Start](tutorials/quickstart.md) Description ...
- [Advanced Usage](tutorials/advanced.md) Description ...
```

- Use `##` headings to create sections in your table of contents
- Use bullet points (`-`) followed by Markdown links to define pages
- Indented bullet points create nested pages in the hierarchy
- Add descriptions to each page and nested page to be used for meta description tags.

### Project structure

The project structure helps to generate the files for a file-based routing system.

```bash
docs/
├── SUMMARY.md
└── api
│   ├── auth.md
│   └── endpoints
│       ├── users.md
│       └── products.md
├──  getting-started.md
├──  index.md
└── tutorials
    ├── quickstart.md
    └── advanced.md
```

### Generate documentation

Based on the project structure, the **query-docs** command generates the necessary files for a file-based routing system.

```bash
query-docs --input ./docs --output ./dist/docs
```

### Documentation Files Generated

The `query-docs` command generates JSON files with all the information required to display your documentation following the documentation project structure.

```bash
dist/docs/
├── toc.json
└── api
│   ├── auth.json
│   └── endpoints
│       ├── users.json
│       └── products.json
├──  getting-started.json
├──  index.json
└── tutorials
    ├── quickstart.json
    └── advanced.json
```

- `*.json`: Contains the related information about a page in the documentation, including its title, description, URL, HTML, markdown, plain text, and navigation.
- `toc.json`: Contains table of contents structure of the documentation, including the hierarchy of pages, titles, and their URLs.

#### Pages Schema

JSON schema used for each page:

- **title**: The title of the current page
- **description**: The description of the current page
- **content**: The HTML content of the current page
- **navigation**: Navigation object with:
  - **previous**: Previous page (title and URL) or null
  - **next**: Next page (title and URL) or null
  - **current**: Current page (title and URL)
- **markdown**: The raw markdown content of the current page
- **path**: The relative URL path of the current page
- **plain_text**: The plain text version of the content with formatting removed
- **metadata**: Additional structured data related to the page (can be null)
  
<details>
<summary>Page Schema</summary>

```json
{
  "type": "object",
  "properties": {
    "content": {
      "type": "string",
      "description": "HTML content of the page"
    },
    "description": {
      "type": "string",
      "description": "Brief summary describing the page content"
    },
    "markdown": {
      "type": "string",
      "description": "Markdown representation of the page content"
    },
    "metadata": {
      "type": ["object", "null"],
      "description": "Additional metadata about the page"
    },
    "navigation": {
      "type": "object",
      "description": "Navigation links related to the current page",
      "properties": {
        "current": {
          "type": "object",
          "properties": {
            "title": { "type": "string" },
            "url": { "type": "string" }
          },
          "required": ["title", "url"]
        },
        "next": {
          "type": "object",
          "properties": {
            "title": { "type": "string" },
            "url": { "type": "string" }
          },
          "required": ["title", "url"]
        },
        "previous": {
          "type": "object",
          "properties": {
            "title": { "type": "string" },
            "url": { "type": "string" }
          },
          "required": ["title", "url"]
        }
      },
      "required": ["current", "next", "previous"]
    },
    "path": {
      "type": "string",
      "description": "Page path/URL"
    },
    "plain_text": {
      "type": "string",
      "description": "Plain text version of the page content without formatting"
    },
    "title": {
      "type": "string",
      "description": "Page title"
    }
  },
  "required": ["content", "description", "markdown", "navigation", "path", "plain_text", "title"]
}
```

</details>

#### Table of Contents Schema

JSON Schema used in the Table of Contents:

- **items**: An object representing the table of contents items.
  - **name**: A string representing the name of each group of page.
  - **items**: An array of objects representing the table of contents items.
  - **group**: A string representing the name of each group of page.
  - **title**: A string representing the title of each page.
  - **url**: A string representing the URL of each page.
  - **level**: A number representing the level of each page in the hierarchy.
  - **children**: An array of objects representing the child pages of each page.

<details>
<summary>Table of Contents Schema</summary>

```json
{
  "type": "object",
  "properties": {
    "items": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "group": {
                "type": "string"
              },
              "title": {
                "type": "string"
              },
              "url": {
                "type": "string"
              },
              "level": {
                "type": "integer"
              },
              "children": {
                "type": "array",
                "items": {
                  "type": "object",
                  "properties": {
                    "group": {
                      "type": "string"
                    },
                    "title": {
                      "type": "string"
                    },
                    "url": {
                      "type": "string"
                    },
                    "level": {
                      "type": "integer"
                    },
                    "children": {
                      "type": "array",
                      "items": {}
                    }
                  },
                  "required": ["group", "title", "url", "level", "children"]
                }
              }
            },
            "required": ["group", "title", "url", "level", "children"]
          }
        }
      },
      "required": ["name", "items"]
    }
  },
  "required": ["items"]
}
```

</details>

### Markdown Syntax

Markdown is a lightweight markup language that allows you to format text using simple syntax. It is widely used for creating README files, documentation, and other types of content.

#### Headings

Markdown supports six levels of headings, which are denoted by one to six `#` symbols at the beginning of a line.

```markdown
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6
```

Alternatively, for H1 and H2, you can use underlines:

```markdown
# Heading 1

## Heading 2
```

<div aria-hidden="true">
Examples:

# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6

</div>

#### Emphasis

Markdown supports two types of emphasis: bold and italic.

```markdown
**Bold text** or **Bold text**
_Italic text_ or _Italic text_
**_Bold and italic text_**
```

Examples:

<div aria-hidden="true">
**Bold text** or __Bold text__

_Italic text_ or _Italic text_

**_Bold and italic text_**

</div>

#### Lists

Markdown supports two types of lists: ordered and unordered.

```markdown
1. Ordered list item 1
2. Ordered list item 2

- Unordered list item 1
- Unordered list item 2
```

You can also use `*` or `+` for unordered lists:

```markdown
- Unordered list with asterisks

* Unordered list with plus signs
```

<div aria-hidden="true">
Example:

1. Ordered list item 1
2. Ordered list item 2

- Unordered list item 1
- Unordered list item 2
</div>

#### Task Lists

GitHub-flavored Markdown supports task lists:

```markdown
- [ ] Incomplete task
- [x] Complete task
```

<div aria-hidden="true">
Example:

- [ ] Incomplete task
- [x] Complete task
</div>

#### Links

Markdown supports links to other pages or external websites.

```markdown
[Link text](https://example.com)
```

You can also use reference-style links:

```markdown
[Link text][reference]

[reference]: https://example.com
```

Autolinks are supported:

```markdown
<https://example.com>
```

<div aria-hidden="true">
Example:

<https://example.com>

</div>

#### Images

Markdown supports images using the following syntax:

```markdown
![Alt text](image.jpg)
```

Reference-style images:

```markdown
![Alt text][image-ref]

[image-ref]: image.jpg
```

<div aria-hidden="true">
Example:

![Query Logo](https://raw.githubusercontent.com/gc-victor/query/main/query-logo.svg)

</div>

#### Tables

Markdown supports tables using the following syntax:

```markdown
| Column 1 | Column 2 |
| -------- | -------- |
| Row 1    | Row 1    |
| Row 2    | Row 2    |
```

<div aria-hidden="true">
Example:

| Column 1 | Column 2 |
| -------- | -------- |
| Row 1    | Row 1    |
| Row 2    | Row 2    |

</div>

#### Code Blocks

Markdown supports code blocks using the following syntax:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

Or using tildes:

```markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
```

Indented code blocks:

```markdown
    # This is a code block
    print("Hello")
```

Inline code uses backticks:

```markdown
Use `println!()` for output
```

<div aria-hidden="true">
Example:

```javascript
function main() {
  console.log("Hello, world!");
}
```

And

Use `println!()` for output

</div>

#### Blockquotes

```markdown
> This is a blockquote
> It can span multiple lines
>
> And paragraphs
```

<div aria-hidden="true">
Example:

> This is a blockquote
> It can span multiple lines
>
> And paragraphs

</div>

#### Thematic Breaks

Horizontal rules can be created using three or more hyphens, asterisks, or underscores:

```markdown
---

---

---
```

<div aria-hidden="true">
Example:

---

---

---

</div>

#### Footnotes

```markdown
Here is a footnote reference[^1]

[^1]: This is the footnote content.
```

<div aria-hidden="true">
Example:

Here is a footnote reference[^1]

[^1]: This is the footnote content.

</div>

#### Strikethrough

```markdown
~~Strikethrough text~~
```

<div aria-hidden="true">
Example:

~~Strikethrough text~~

</div>

#### Line Breaks

For a hard line break, end a line with two or more spaces, or with a backslash:

```markdown
First line
Second line

First line\
Second line
```

<div aria-hidden="true">
Example:

First line
Second line

First line\
Second line

</div>

#### Heading Attributes

You can add attributes to headings:

```markdown
# Custom heading {.class-name #custom-id}
```

<div aria-hidden="true">
Example:

# Custom heading {.class-name #custom-id}

</div>

#### Block Quote Tags

GitHub-flavored Markdown supports blockquote tags:

```markdown
<div class="markdown-alert">

> [!NOTE]
>
> ### NOTE!{.markdown-alert-title .font-cal}
>
> This is a note message

</div>

<div class="markdown-alert">

> [!WARNING]
>
> ### WARNING!{.markdown-alert-title .font-cal}
>
> This is a warning message

</div>

<div class="markdown-alert">

> [!IMPORTANT]
>
> ### IMPORTANT!{.markdown-alert-title .font-cal}
>
> This is an important message

</div>
```

<div aria-hidden="true">
Example:

<div class="markdown-alert">

> [!NOTE]
>
> ### NOTE!{.markdown-alert-title .font-cal}
>
> This is a note message

</div>

<div class="markdown-alert">

> [!WARNING]
>
> ### WARNING!{.markdown-alert-title .font-cal}
>
> This is a warning message

</div>

</div>

#### Metadata Blocks

YAML frontmatter can be included at the beginning of a document:

```markdown
---
title: Document Title
author: Author Name
date: 2023-01-01
---
```

It will be added as a metadata object to use on your template.

Example:

```json
{
    ...

    matadata: {
      "title": "Document Title",
      "author": "Author Name",
      "date": "2023-01-01"
    }
    ...
}
```

#### Paragraphs

Paragraphs are separated by blank lines:

```markdown
This is paragraph one.

This is paragraph two.
```

<div aria-hidden="true">
Example:

This is paragraph one.

This is paragraph two.

</div>

#### Escaping Characters

You can escape special Markdown characters using a backslash:

```markdown
\*This text is not italic\*
```

Escapable characters include:

```
\ ` * _ { } [ ] ( ) # + - . ! |
```

#### HTML

Most Markdown parsers, including pulldown-cmark, allow inline HTML:

```markdown
<div class="custom">
  Custom HTML content
</div>
```

#### Comments

HTML comments can be used in Markdown:

```markdown
<!-- This is a comment that won't appear in the rendered output -->
```

#### Smart Punctuation

Some Markdown processors support smart punctuation, converting straight quotes to curly quotes, -- to en-dashes, and --- to em-dashes:

```markdown
"Quote" becomes "Quote"
-- becomes –
--- becomes —
```

<div aria-hidden="true">
Example:

"Quote" becomes "Quote"

-- becomes –

--- becomes —

</div>

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
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ title }}</title>
    <meta name="description" content="{{ description }}" />
    <link rel="stylesheet" href="styles.css" />
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
      <article>{{ content }}</article>
    </main>

    <nav class="pagination">
      {% if navigation.previous %}
      <a href="{{ navigation.previous.url }}" class="prev">← {{ navigation.previous.title }}</a>
      {% endif %} {% if navigation.next %}
      <a href="{{ navigation.next.url }}" class="next">{{ navigation.next.title }} →</a>
      {% endif %}
    </nav>

    {% if search_enabled %}
    <script src="search.js"></script>
    {% endif %}
  </body>
</html>
```
