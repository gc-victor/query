# Generate

Query's generate is a tool that helps you create a set of files using a simple command that represents a table's structure. It lets you quickly and easily create the needed files without writing everything from scratch.

Example:

```sh
query generate blog.sql post title:string content:text
```

Format:

```sh
query generate <DATABA> <TABLE> <COLUMNS[COLUMN:TYPE]>
```

## Column Types

The following table illustrates the mapping between Column Types, TypeScript, and SQLite data types:

| ColumnType | TypeScript | SQLite  |
|------------|------------|---------|
| blob       | Blob       | BLOB    |
| boolean    | boolean    | BOOLEAN |
| number     | number     | INTEGER |
| integer    | number     | INTEGER |
| float      | number     | REAL    |
| real       | number     | REAL    |
| timestamp  | string     | INTEGER DEFAULT (strftime('%s', 'now')) |
| string     | string     | TEXT    |
| text       | string     | TEXT    |
| uuid       | string     | TEXT UNIQUE CHECK ({column_name} != '') DEFAULT (uuid()) |
| foreign    | number     | FOREIGN KEY ({column_name}) REFERENCES {parent_table} (id/uuid) |

> [!NOTE]
> The `foreign` type is a special type that creates a foreign key in the column. The format to use it is `column_name:foreign`, where the `column_name`, should have the format `<parent_table>_id` or `<parent_table>_uuid`.

## How it works

The generate does two things:

- Generates the database migrations files to update your database
- Generates a set of files based on templates

### Database migrations

The migration generated will use the command to create the table and the columns. The migration will be stored in the **/migrations** folder inside a folder with the database name (Ex. blog.sql). It will generate two files with the format of **`<version>_<name>_<type>.sql`**. The version will have the format of YYYYMMDDHHMMSS, the name should be in the format of `<name>_<description>`, and the types will be **up** and **down**.

You can find more information about migrations in the [Migration](/docs/cli/migration.md) section.

### Templates

The templates used to generate files are stored in the **/templates** folder or a custom folder specified in the Query's config file.

```toml
[structure]
templates_folder = other-template-folder
```

Query uses a basic template system that we will describe in detail below.

There are some dynamic variables based on the command params that you can use to generate the file content:

- **{{ database }}**: The database where the migration will be executed
- **{{ table }}**<sup>1</sup>: The name of the table
- **{{ columnsLength }}**: The number of the columns
- **{{ columns }}**: The list of columns specified
  - **{{ columnIndex }}**: The index value in the loop
  - **{{ columnFirst }}**: The first column in the loop
  - **{{ columnLast }}**: The last column in the loop
  - **{{ columnName }}**<sup>2</sup> <sup>1</sup>: The name of the column
  - **{{ columnTypeMatchTS }}**: The match of the type of the column with the TypeScript type
  - **{{ columnsListOfUniqueTSTypes }}**: A list of the matches between column type and TypeScript type in lowercase
  - **{{ columnType }}**<sup>2</sup> <sup>1</sup>: The type of the column

<sub>1 The table, the columnName, and the columnType have name variants you can use in your templates.</sub>

<sub>2 To get the columnName and columnType, it is required to iterate over the columns.</sub>

As we have commented, you can use some name variants in your templates for the table, columnName, and columnType. The name variants are based on the command that you will use to generate the files.

**Variants:**

- **camelCase** (Ex. testName)
- **hyphenCase** (Ex. test-name)
- **snakeCase** (Ex. test_name)
- **dotCase** (Ex. test.name)
- **pathCase** (Ex. test/name)
- **constantCase** (Ex. TEST_NAME)
- **pascalCase** (Ex. TestName)
- **capitalCase** (Ex. Test Name)
- **lowerCase** (Ex. test name)
- **sentenceCase** (Ex. Test name)
- **upperCase** (Ex. TEST NAME)
- **upperCaseFirst** (Ex. Test name)
- **lowerCaseFirst** (Ex. test name)

**Variables:**

```tmpl
{{ tableCamelCase }}
{{ tableHyphenCase }}
{{ tableSnakeCase }}
{{ tableDotCase }}
{{ tablePathCase }}
{{ tableConstantCase }}
{{ tablePascalCase }}
{{ tableCapitalCase }}
{{ tableLowerCase }}
{{ tableSentenceCase }}
{{ tableUpperCase }}
{{ tableUpperCaseFirst }}
{{ tableLowerCaseFirst }}
{{ columnNameCamelCase }}
{{ columnNameHyphenCase }}
{{ columnNameSnakeCase }}
{{ columnNameDotCase }}
{{ columnNamePathCase }}
{{ columnNameConstantCase }}
{{ columnNamePascalCase }}
{{ columnNameCapitalCase }}
{{ columnNameLowerCase }}
{{ columnNameSentenceCase }}
{{ columnNameUpperCase }}
{{ columnNameUpperCaseFirst }}
{{ columnNameLowerCaseFirst }}
{{ columnTypeCamelCase }}
{{ columnTypeHyphenCase }}
{{ columnTypeSnakeCase }}
{{ columnTypeDotCase }}
{{ columnTypePathCase }}
{{ columnTypeConstantCase }}
{{ columnTypePascalCase }}
{{ columnTypeCapitalCase }}
{{ columnTypeLowerCase }}
{{ columnTypeSentenceCase }}
{{ columnTypeUpperCase }}
{{ columnTypeUpperCaseFirst }}
{{ columnTypeLowerCaseFirst }}
```

The template system provides two operations to use in your templates:

**If:**

```html
{% if table == "post" %}
  <p>This is a Post.</p>
{% else %}
  <p>This isn't a Post.</p>
{% endif %}
```

**For:**

```html
{% for column in columns %}
  <p>{% column.columnName %}</p>
{% endfor %}
```

With the previous information, you can create a set of files based on the table's schema. These files should be placed in the templates folder, with the folder structure used to generate files in their respective locations. The templates folder structure should match that of the *functions_folder*, which is typically configured as */src*, although you will need to configure it yourself. You can find more information about the configuration process in the [Configuration](/docs/configuration.md) section.

Example from the [query-app](https://github.com/gc-victor/query-app) project:

API:

```sh
templates
├── api
│   ├── admin
│   │   ├── login
│   │   │   └── __table__.index.ts
│   │   └── __table__
│   │       ├── delete.index.ts
│   │       ├── get.index.ts
│   │       ├── post.index.ts
│   │       ├── put.index.ts
│   │       └── uuid
│   │           └── get.[slug].ts
│   └── __table__
│       ├── delete.index.ts
│       ├── get.index.ts
│       ├── post.index.ts
│       ├── put.index.ts
│       └── uuid
│           └── get.[slug].ts
└── ...
```

Pages:

```sh
templates
├── pages
│   ├── admin
│   │   ├── components
│   │   │   └── ...
│   │   ├── get.index.ts
│   │   ├── login
│   │   │   └── ...
│   │   ├── __table__
│   │   │   ├── get.index.tsx
│   │   │   ├── island
│   │   │   │   └── __table__.island.ts
│   │   │   ├── __table__.form.view.tsx
│   │   │   └── __table__.view.tsx
│   │   └── utils
│   │       └── ..
│   ├── components
│   │   └── ..
│   ├── get.index.tsx
│   ├── layouts
│   │   └── ...
│   ├── __table__
│   │   ├── excerpt.tsx
│   │   ├── get.index.tsx
│   │   └── [slug]
│   │       ├── get.index.tsx
│   │       └── __table__.tsx
│   └── styles.css
└── ...
```

Notice that **"\_\_table\_\_"** is a placeholder for that will be replaced by the table name of the command.
