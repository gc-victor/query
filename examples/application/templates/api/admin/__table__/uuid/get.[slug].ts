import { QUERY_API_QUERY } from "@/config/server/server.constants";
import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        const session = await getAdminUserSession(req);
        const isExpired = await adminUserSession.isExpired(session);

        if (isExpired) {
            await adminUserSession.refresh(session);
        }

        const { token } = await adminUserSession.load(session);

        const uuid = req.url.split("/").pop();
        const query = "SELECT * FROM  {{ tableSnakeCase }} WHERE uuid = :uuid;";
        const params = {
            ":uuid": uuid,
        };

        const response = await fetcher(QUERY_API_QUERY, {
            method: Method.POST,
            body: JSON.stringify({ db_name: {{ tableConstantCase }}_DATABASE, query, params }),
            headers: {
                [AUTHORIZATION_REQUEST]: `Bearer ${token}`,
                [CONTENT_TYPE_REQUEST]: "application/json",
            },
        });

        return ok(JSON.stringify(response.json));
    } catch (e) {
        return handleRequestError(e as Error);
    }
}
