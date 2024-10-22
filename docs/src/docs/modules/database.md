# Database Module

Query provides a powerful and intuitive database module through the `Database` class. This module allows you to interact with SQLite databases directly in your Query functions, offering both read and write operations with support for transactions, prepared statements, and query caching.

## Basic Usage

```javascript
import { Database } from "query:database";

// Create or connect to a database
const db = new Database("example.sql");

// Execute a query
const results = await db.query(
    "SELECT * FROM users WHERE age > ?",
    [18]
);
```

## API Reference

### Constructor

#### `new Database(dbName)`

Creates a new database connection or connects to an existing database.

| Parameter | Type | Description |
|-----------|------|-------------|
| dbName | string | Name of the database file (e.g., "example.sql") |

### Methods

#### `query(sql, params?)`

Executes an SQL query with optional parameters.

| Parameter | Type | Description |
|-----------|------|-------------|
| sql | string | SQL query to execute |
| params | array \| object | Query parameters (optional) |

Returns: Promise resolving to query results

## Query Parameters

### Array Parameters

Use `?` placeholders for array parameters:

```javascript
const results = await db.query(
    "SELECT * FROM users WHERE age > ? AND city = ?",
    [18, "New York"]
);
```

### Named Parameters

Use `:name`, `$name`, or `@name` placeholders for object parameters:

```javascript
const results = await db.query(
    "SELECT * FROM users WHERE age > :age AND city = :city",
    { ":age": 18, ":city": "New York" }
);
```

## Examples

### Creating a Table

```javascript
const db = new Database("example.sql");

await db.query(`
    CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT UNIQUE,
        age INTEGER,
        created_at INTEGER DEFAULT (strftime('%s', 'now'))
    )
`);
```

### Inserting Data

```javascript
// Single insert
await db.query(
    "INSERT INTO users (name, email, age) VALUES (?, ?, ?)",
    ["John Doe", "john@example.com", 25]
);

// Multiple inserts using named parameters
await db.query(`
    INSERT INTO users (name, email, age)
    VALUES (:name1, :email1, :age1),
           (:name2, :email2, :age2)
`, {
    ":name1": "John",  ":email1": "john@example.com",  ":age1": 25,
    ":name2": "Jane",  ":email2": "jane@example.com",  ":age2": 23
});
```

### Selecting Data

```javascript
// Basic select
const allUsers = await db.query("SELECT * FROM users");

// With conditions
const activeUsers = await db.query(
    "SELECT * FROM users WHERE active = ? AND age > ?",
    [true, 18]
);

// With joins
const userPosts = await db.query(`
    SELECT users.name, posts.title
    FROM users
    JOIN posts ON users.id = posts.user_id
    WHERE users.id = ?
`, [userId]);
```

### Updating Data

```javascript
await db.query(
    "UPDATE users SET age = :age WHERE id = :id",
    { ":age": 26, ":id": 1 }
);
```

### Deleting Data

```javascript
await db.query(
    "DELETE FROM users WHERE id = ?",
    [userId]
);
```

## Best Practices

1. **Use Prepared Statements**: Always use parameterized queries to prevent SQL injection:

```javascript
// Good
await db.query("SELECT * FROM users WHERE id = ?", [userId]);

// Bad - Don't do this!
await db.query(`SELECT * FROM users WHERE id = ${userId}`);
```

1. **Error Handling**: Implement proper error handling for database operations:

```javascript
try {
    const result = await db.query("SELECT * FROM users");
} catch (error) {
    console.error("Database error:", error);
    // Handle error appropriately
}
```
