import { QUERY_API_QUERY } from "@/config/server/server.constants";
import { POST_DATABASE } from "@/config/shared/post.constants";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { bad_request, ok } from "@/lib/server/responses";
import { tokenService, validateToken } from "@/lib/server/token";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        validateToken("post", req);

        const token = await tokenService.load("post", req);

        const { uuid } = await req.json();

        if (!uuid) {
            throw bad_request("The uuid is required.");
        }

        const query = "DELETE FROM post WHERE uuid = :uuid;";
        const params = {
            ":uuid": uuid,
        };

        const response = await fetcher(QUERY_API_QUERY, {
            method: Method.POST,
            body: JSON.stringify({ db_name: POST_DATABASE, query, params }),
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
