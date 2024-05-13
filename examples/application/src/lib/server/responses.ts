import { ResponseError } from "./response-error";
import {
    BAD_REQUEST_CODE,
    CREATED_CODE,
    INTERNAL_SERVER_ERROR_CODE,
    INTERNAL_SERVER_ERROR_MESSAGE,
    NOT_FOUND_CODE,
    NOT_IMPLEMENTED_CODE,
    NO_CONTENT_CODE,
    OK_CODE,
    UNAUTHORIZED_CODE,
} from "./status";

export function ok(body: string): Response {
    return new Response(body, { status: OK_CODE });
}

export function created(): Response {
    return new Response("", { status: CREATED_CODE });
}

export function no_content(): Response {
    return new Response("", { status: NO_CONTENT_CODE });
}

export function unauthorized() {
    throw new ResponseError(JSON.stringify({ message: "", status: UNAUTHORIZED_CODE }));
}

export function not_found() {
    throw new ResponseError(JSON.stringify({ message: "", status: NOT_FOUND_CODE }));
}

export function not_implemented() {
    throw new ResponseError(JSON.stringify({ message: "", status: NOT_IMPLEMENTED_CODE }));
}

export function bad_request(message: string) {
    throw new ResponseError(JSON.stringify({ message, status: BAD_REQUEST_CODE }));
}

export function internal_server_error(error?: unknown) {
    throw new ResponseError(JSON.stringify({ message: INTERNAL_SERVER_ERROR_MESSAGE, status: INTERNAL_SERVER_ERROR_CODE }));
}
