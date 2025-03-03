import { ResponseError } from "./response-error";
import {
    BAD_REQUEST_CODE,
    CREATED_CODE,
    INTERNAL_SERVER_ERROR_CODE,
    INTERNAL_SERVER_ERROR_MESSAGE,
    NOT_FOUND_CODE,
    NOT_FOUND_MESSAGE,
    NOT_IMPLEMENTED_CODE,
    NOT_IMPLEMENTED_MESSAGE,
    NO_CONTENT_CODE,
    OK_CODE,
    UNAUTHORIZED_CODE,
    UNAUTHORIZED_MESSAGE,
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
    throw new ResponseError(UNAUTHORIZED_MESSAGE, UNAUTHORIZED_CODE);
}

export function not_found() {
    throw new ResponseError(NOT_FOUND_MESSAGE, NOT_FOUND_CODE);
}

export function not_implemented() {
    throw new ResponseError(NOT_IMPLEMENTED_MESSAGE, NOT_IMPLEMENTED_CODE);
}

export function bad_request(message: string) {
    throw new ResponseError(message, BAD_REQUEST_CODE);
}

export function internal_server_error(error?: unknown) {
    throw new ResponseError(INTERNAL_SERVER_ERROR_MESSAGE, INTERNAL_SERVER_ERROR_CODE);
}
