export function htmlResponse(content: JSX.Element | string, headers?: HeadersInit): Response {
    return new Response(`<!DOCTYPE html>${content}`, {
        status: 200,
        headers: {
            "Content-Type": "text/html; charset=utf-8",
            ...headers,
        },
    });
}
