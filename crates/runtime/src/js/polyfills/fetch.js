// @see: https://developer.mozilla.org/en-US/docs/Web/API/fetch
// @see: https://developer.mozilla.org/en-US/docs/Web/API/fetch#resource
// @see: https://developer.mozilla.org/en-US/docs/Web/API/fetch#options
async function fetch(resource, options = {}) {
    const resourceBody = typeof resource === "string" ? null : resource.body;
    const optionsBody = options.body;

    if (resourceBody) {
        resource.body = await getBody(resourceBody);
    } else if (optionsBody) {
        options.body = await getBody(optionsBody);
    }

    const response = await ___fetcher(resource, options);

    response.headers = new Headers(response.headers);
    const contentType = response.headers.get("content-type");

    let body = new Uint8Array(response.body).buffer;

    if (contentType?.includes("application") && !contentType?.includes("application/json")) {
        body = new Blob([body], { type: contentType });
    } else {
        body = new TextDecoder().decode(body);
    }

    return Promise.resolve(
        new Response(body || "", {
            status: response.status,
            url: resource.url,
            headers: response.headers,
        }),
    );
}

globalThis.fetch = fetch;

async function getBody(body) {
    if (body instanceof Blob) {
        return new Uint8Array(await body.arrayBuffer());
    }

    if (body instanceof FormData) {
        return new TextEncoder().encode(body.toString());
    }

    if (body instanceof URLSearchParams) {
        return new TextEncoder().encode(body.toString());
    }

    if (body instanceof ArrayBuffer) {
        return new Uint8Array(body);
    }

    if (body instanceof ReadableStream) {
        return new Uint8Array(await getBodyStream(body));
    }

    return new TextEncoder().encode(body);
}

async function getBodyStream(body) {
    const reader = body.getReader();
    const chunks = [];

    while (true) {
        const { done, value } = await reader.read();

        if (done) {
            break;
        }

        chunks.push(value);
    }

    return chunks;
}
