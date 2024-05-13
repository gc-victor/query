import { parse } from "valibot";

import { LoginValidation } from "@/api/admin/login/login.validation";
import { QUERY_API_USER_TOKEN_VALUE } from "@/config/server/server.constants";
import { setAdminUserSession } from "@/lib/server/admin-user-session";
import { cors } from "@/lib/server/cors";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        const formData = await req.formData();
        const email = formData.get("email");
        const password = formData.get("password");

        parse(LoginValidation, { email, password });

        const response = await fetcher(QUERY_API_USER_TOKEN_VALUE, {
            method: Method.POST,
            body: JSON.stringify({ email, password }),
            headers: {
                [CONTENT_TYPE_REQUEST]: "application/json",
            },
        });

        const json = response.json as unknown as { data: { token: string }[] };
        const res = ok(JSON.stringify(json));

        await setAdminUserSession(json.data[0].token, res);
        cors(res);

        return res;
    } catch (e: unknown) {
        return handleRequestError(e as Error);
    }
}
