import { Database } from "bun:sqlite";

const db = new Database(`${process.env.QUERY_SERVER_DBS_PATH}/kv.sql`, { create: true });

db.run("CREATE TABLE IF NOT EXISTS kv (key TEXT NOT NULL UNIQUE, value TEXT);");

const server = Bun.serve({
    port: 3001,
    async fetch(req) {
        const { pathname } = new URL(req.url);

        const query = db.query("SELECT * FROM kv").all();

        if (pathname === "/proxy" && req.method === "GET") return new Response(`${JSON.stringify(query)}`);
        if (pathname === "/proxy" && req.method === "POST") {
            const body = await req.json();
            
            db.run("INSERT OR IGNORE INTO kv (key, value) VALUES (?, ?)", body.key, body.value);

            return new Response("Success!");
        }

        return new Response("Not found", { status: 404 });
    },
});

console.log(`Listening on http://localhost:${server.port}`);
