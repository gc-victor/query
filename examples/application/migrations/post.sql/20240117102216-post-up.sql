-- Create the post table
CREATE TABLE IF NOT EXISTS post (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT UNIQUE CHECK (uuid != '') DEFAULT (uuid ()) NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    image_url TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime ('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime ('%s', 'now'))
);

-- Create the trigger to update the updated_at column
CREATE TRIGGER IF NOT EXISTS trigger_post_update AFTER
UPDATE ON post BEGIN
UPDATE post
SET
    updated_at = (strftime ('%s', 'now'))
WHERE
    id = OLD.id;
END;

-- Insert a post record
INSERT INTO post (
    title,
    content,
    slug,
    image_url,
    created_at,
    updated_at
)
VALUES
    (
        'Exploring the Power of SQLite: A Dive into the World of Lightweight Databases',
        replace (
            replace (
                '<section>\r\n        <h2>Introduction</h2>\r\n        <p>In the vast landscape of databases, one name stands out for its simplicity, efficiency, and versatility - SQLite. Whether you''re a seasoned developer or just starting your journey into the world of databases, SQLite has something to offer. In this blog post, we''ll take a closer look at the power and potential of SQLite, uncovering its features, use cases, and why it''s the go-to choice for many developers.</p>\r\n    </section>\r\n\r\n    <section>\r\n        <h2>Understanding SQLite</h2>\r\n        <p>SQLite is a self-contained, serverless, and zero-configuration database engine. What sets it apart is its lightweight nature, making it a perfect fit for embedded systems, mobile applications, and small to medium-sized projects. Despite its size, SQLite doesn''t compromise on functionality; it supports a significant subset of SQL standards and provides ACID-compliant transactions.</p>\r\n\r\n        <h3>Key Features of SQLite</h3>\r\n        <ul>\r\n            <li><strong>Zero Configuration:</strong> Unlike other database systems, SQLite doesn''t require a separate server process or setup. It operates directly on the file system, simplifying the deployment process.</li>\r\n            <li><strong>Cross-Platform:</strong> SQLite is compatible with various operating systems, including Windows, macOS, Linux, and even embedded systems. This cross-platform support makes it easy for developers to work seamlessly across different environments.</li>\r\n            <li><strong>Transactional Support:</strong> SQLite ensures the integrity of your data with support for ACID transactions. This means that database operations are Atomic, Consistent, Isolated, and Durable, providing a reliable foundation for your applications.</li>\r\n            <li><strong>Self-Contained:</strong> The entire database is contained in a single ordinary file on the host system. This makes it easy to manage, backup, and transfer, simplifying the overall development and deployment process.</li>\r\n        </ul>\r\n    </section>\r\n\r\n    <section>\r\n        <h2>Use Cases for SQLite</h2>\r\n        <ul>\r\n            <li><strong>Mobile Applications:</strong> SQLite is widely used in mobile app development due to its lightweight nature and seamless integration with platforms like Android and iOS. Many popular apps, including WhatsApp and Firefox, leverage SQLite for data storage.</li>\r\n            <li><strong>Embedded Systems:</strong> Its small footprint makes SQLite an excellent choice for embedded systems and IoT devices, where resources are limited. The ability to operate without a separate server makes it ideal for scenarios with constrained hardware.</li>\r\n            <li><strong>Prototyping and Testing:</strong> Developers often use SQLite during the prototyping and testing phases of a project. Its simplicity and ease of use allow for quick iteration and testing of database-related functionality.</li>\r\n        </ul>\r\n    </section>\r\n\r\n    <section>\r\n        <h2>Conclusion</h2>\r\n        <p>SQLite may not be the heavyweight champion of the database world, but its lightweight and versatile nature make it a strong contender for various applications. Whether you''re building a mobile app, an embedded system, or just experimenting with a new project, SQLite''s simplicity, efficiency, and cross-platform support make it a reliable choice. As you explore the vast realm of databases, consider adding SQLite to your toolkit and unlock the potential of this compact yet powerful database engine.</p>\r\n    </section>',
                '\r',
                char(13)
            ),
            '\n',
            char(10)
        ),
        '/exploring-the-power-of-sqlite-a-dive-into-the-world-of-lightweight-databases',
        -- This is a temporary image URL as the image will be added in the admin post form
        -- and the image will stored as `post/cache/_ff7fc93a-c2c8-44b5-87f5-d9348ef38cec.webp`
        'public/images/cache/_ff7fc93a-c2c8-44b5-87f5-d9348ef38cec.webp',
        1707313321,
        1707313321
    );
