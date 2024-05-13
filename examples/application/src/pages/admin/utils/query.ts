import { QUERY_API_QUERY } from "@/config/server/server.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { cors } from "@/lib/server/cors";
import { fetcher } from "@/lib/server/fetcher";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";

export async function apiQuery(req: Request): Promise<Response> {
    const { db_name, query, params } = await req.json();

    const sessionToken = await getAdminUserSession(req);
    const isExpired = await adminUserSession.isExpired(sessionToken);

    if (isExpired) {
        adminUserSession.refresh(sessionToken);
    }

    const session = await adminUserSession.load(sessionToken);

    const response = await fetcher(QUERY_API_QUERY, {
        method: Method.POST,
        body: JSON.stringify({ db_name, query, params }),
        headers: {
            [AUTHORIZATION_REQUEST]: `Bearer ${session.token}`,
            [CONTENT_TYPE_REQUEST]: "application/json",
        },
    });

    const res = ok(JSON.stringify(response.json));

    cors(res);
    adminUserSession.refresh(sessionToken);

    return res;
}
