globalThis.___handleResponse = async function () {
    const response = await ___handleRequestWrapper();
    const body = await response.arrayBuffer();
    const headers = {};

    for (const [key, value] of response.headers) {
        headers[key] = value;
    }

    return {
        body: body,
        bodyUsed: response.bodyUsed,
        headers: headers,
        ok: response.ok,
        redirected: response.redirected,
        status: response.status,
        statusText: response.statusText,
        type: response.type,
        url: response.url,
    };
}