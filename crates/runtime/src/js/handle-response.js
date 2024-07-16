export async function ___handleResponse(headers, method, url, body) {
    try {
        const options = {
            headers: headers,
            method: method,
            url: url,
        };

        if (body && method !== "GET" && method !== "HEAD") {
            options.body = body;
        }

        const response = await ___handleRequest(new Request(url, options));

        return {
            body: await response.text(),
            bodyUsed: response.bodyUsed,
            // Convert Headers to a plain object
            headers: Object.fromEntries(response.headers),
            ok: response.ok,
            redirected: response.redirected,
            status: response.status,
            statusText: response.statusText,
            type: response.type,
            url: response.url,
        };
    } catch (e) {
        console.error("error", `${e.message}\n${e.stack || ""}`);

        return {
            body: "",
            bodyUsed: false,
            headers: {},
            ok: false,
            redirected: false,
            status: 500,
            statusText: "Internal Server Error",
            type: "error",
            url: "",
        };
    }
}
