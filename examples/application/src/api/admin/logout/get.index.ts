import { PAGE_ADMIN_LOGIN_PATH } from "@/config/shared/shared.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { SET_COOKIE_RESPONSE } from "@/lib/server/header";
import { FOUND_CODE, FOUND_MESSAGE } from "@/lib/server/status";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        const session = await getAdminUserSession(req);
        await adminUserSession.clear(session);
    } catch {}

    const url = new URL(req.url);

    return new Response(null, {
        status: FOUND_CODE,
        statusText: FOUND_MESSAGE,
        headers: {
            location: url.origin + PAGE_ADMIN_LOGIN_PATH,
            [SET_COOKIE_RESPONSE]: "session=; Path=/; Max-Age=0; HttpOnly; Secure;",
        },
    });
}
