import { ValiError, flatten } from "valibot";

import { AdminUserSessionError } from "@/lib/server/admin-user-session";
import { ResponseError } from "@/lib/server/response-error";
import { TokenError } from "@/lib/server/token";
import {
    BAD_REQUEST_CODE,
    INTERNAL_SERVER_ERROR_CODE,
    INTERNAL_SERVER_ERROR_MESSAGE,
    UNAUTHORIZED_CODE,
    UNAUTHORIZED_MESSAGE,
} from "./status";

export function handleRequestError(e: Error): Response {
    if (e instanceof ResponseError) {
        const error = e as ResponseError;

        return new Response(String(error?.cause?.message) || INTERNAL_SERVER_ERROR_MESSAGE, {
            status: Number(error?.cause?.status) || INTERNAL_SERVER_ERROR_CODE,
        });
    }

    if (e instanceof AdminUserSessionError) {
        return new Response(UNAUTHORIZED_MESSAGE, {
            status: UNAUTHORIZED_CODE,
        });
    }

    if (e instanceof TokenError) {
        return new Response(UNAUTHORIZED_MESSAGE, {
            status: UNAUTHORIZED_CODE,
        });
    }

    if (e instanceof ValiError) {
        return new Response(JSON.stringify({ errors: flatten(e).nested }), {
            status: BAD_REQUEST_CODE,
        });
    }

    return new Response(e.message, {
        status: BAD_REQUEST_CODE,
    });
}
