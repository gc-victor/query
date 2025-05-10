import { Database } from "query:database";
import { jsonResponse, handleApiError } from "@/pages/lib/server/response";

export async function handleRequest(req: Request) {
    try {
        const { value } = await req.json();
        
        const db = new Database("counter.sql");
        db.query(
            "UPDATE counter SET value = ?, updated_at = CURRENT_TIMESTAMP WHERE id = 1",
            [value]
        );

        return jsonResponse({ value });
    } catch (e) {
        return handleApiError(e);
    }
}