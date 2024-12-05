import { Database } from "query:database";

export async function handleRequest(req: Request) {
    try {
        const { value } = await req.json();
        
        const db = new Database("counter.sql");
        db.query(
            "UPDATE counter SET value = ?, updated_at = CURRENT_TIMESTAMP WHERE id = 1",
            [value]
        );

        return new Response(JSON.stringify({ value }), {
            status: 200,
            headers: {
                "content-type": "application/json"
            }
        });
    } catch (e) {
        const error = e as Error;
        
        console.error(JSON.stringify({ error: error.message, stack: error.stack }));

        return new Response("Internal Server Error", {
            status: 500,
            headers: {
                "content-type": "application/json"
            }
        });
    }
}