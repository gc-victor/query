import { QUERY_API_QUERY } from "@/config";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { server } from "@/lib/server/server";
import { QUERY_TOKEN } from "./query-token";

export async function query(db_name: string, query: string, params?: Record<string, unknown> | (string | number)[]): Promise<unknown> {
    const response = await server(QUERY_API_QUERY, {
        method: Method.POST,
        body: JSON.stringify({ db_name, query, params }),
        headers: {
            [AUTHORIZATION_REQUEST]: `Bearer ${QUERY_TOKEN}`,
            [CONTENT_TYPE_REQUEST]: "application/json",
        },
    });

    return response.json;
}
