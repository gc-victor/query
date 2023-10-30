globalThis.___handleResponse = async function () {
    const response = await ___handleRequestWrapper();
    const body = await response.arrayBuffer();

    let headers = {};

    response.headers.forEach((value, key) => {
        headers[key] = value;
    });

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