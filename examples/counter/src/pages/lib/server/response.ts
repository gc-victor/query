import { Database } from "query:database";

export function jsonResponse(data: unknown, status = 200) {
    return new Response(JSON.stringify(data), {
        status,
        headers: { "content-type": "application/json" }
    });
}

export function htmlResponse(html: ComponentChildren, status = 200) {
    return new Response(`<!DOCTYPE html>${StringHTML(`<html lang="en">${html}</html>`)}`, {
        status,
        headers: { "content-type": "text/html; charset=utf-8" }
    });
}

export function handleApiError(e: unknown) {
    const error = e instanceof Error ? e : new Error(String(e));
    console.error(JSON.stringify({ 
        error: error.message, 
        stack: error.stack 
    }));
    return jsonResponse({ error: error.message }, 500);
}