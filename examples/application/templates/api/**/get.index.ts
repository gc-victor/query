import { QUERY_API_QUERY } from "@/config/server/server.constants";
import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";
import { tokenService, validateToken } from "@/lib/server/token";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        validateToken("{{ table }}", req);

        const token = await tokenService.load("{{ table }}", req);

        const query = "SELECT * FROM  {{ tableSnakeCase }};";
        const params: never[] = [];

        const response = await fetcher(QUERY_API_QUERY, {
            method: Method.POST,
            body: JSON.stringify({ db_name: {{ tableConstantCase }}_DATABASE, query, params }),
            headers: {
                [AUTHORIZATION_REQUEST]: `Bearer ${token.query_token}`,
                [CONTENT_TYPE_REQUEST]: "application/json",
            },
        });

        return ok(JSON.stringify(response.json));
    } catch (e) {
        return handleRequestError(e as Error);
    }
}
