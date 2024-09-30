const ___response = Symbol();

/**
 * Response
 *
 * The Response interface of the Fetch API represents the response to a request.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/Response
 * @see: https://fetch.spec.whatwg.org/#response-class
 * @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L448
 */
class Response {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/Response
    constructor(body, init = {}) {
        this[___response] = {};

        const headers = new Headers(init.headers || {});
        const status = init.status !== undefined ? init.status : 200;
        const location = headers.get("location");

        if (status < 200 || status > 599) {
            throw new RangeError("Invalid status code");
        }

        if (!headers.has("content-type") && body) {
            // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/Response#body
            const types = {
                "[object FormData]": () => {
                    const boundary = createBoundary();
                    return `multipart/form-data; boundary=${boundary}`;
                },
                "[object Blob]": () => body.type,
                "[object URLSearchParams]": () => "application/x-www-form-urlencoded;charset=UTF-8",
                "[object String]": () => "text/plain;charset=UTF-8",
            };

            const type = types[Object.prototype.toString.call(body)];

            headers.set("content-type", type?.() || null);
        }

        // https://fetch.spec.whatwg.org/#null-body-status
        const updatedBody = [101, 103, 204, 205, 304].includes(init.status) ? null : body;

        this[___response].body = typeof updatedBody === "string" ? new TextEncoder().encode(updatedBody) : updatedBody || null;
        this[___response].bodyUsed = false;
        this[___response].headers = headers;
        this[___response].ok = status >= 200 && status < 300;
        this[___response].redirected = !!location;
        this[___response].status = status;
        this[___response].statusText = init.statusText === undefined ? statusTextList[this.status] : init.statusText;
        this[___response].type = "basic";
        this[___response].url = location || "";
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/error
    static error() {
        const response = new Response(null, { status: 0, statusText: "" });

        response.type = "error";
        response.headers.immutable = true;

        return response;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect
    // @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location
    // @see: https://fetch.spec.whatwg.org/#redirect-status
    static redirect(url, statusCode = 307) {
        if ([301, 302, 303, 307, 308].indexOf(statusCode) === -1) {
            throw new RangeError("Invalid redirect status code.");
        }

        const response = new Response(null, {
            status: statusCode,
            statusText: statusTextList[statusCode],
            headers: {
                Location: new URL(url).toString(),
            },
        });

        response.type = "default";

        return response;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/body
    get body() {
        if (this[___response].body === null) return null;
        if (this[___response].body instanceof ReadableStream) return this[___response].body;

        const stream = new TransformStream();
        const writer = stream.writable.getWriter();

        writer.write(this[___response].body);
        writer.close();

        return stream.readable;
    }

    // read-only
    set body(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed
    get bodyUsed() {
        return this[___response].bodyUsed;
    }

    // read-only
    set bodyUsed(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer
    async arrayBuffer() {
        if (this[___response].bodyUsed) {
            throw new TypeError("Failed to execute 'arrayBuffer': body stream already read");
        }

        let body = this[___response].body;

        if (body instanceof ReadableStream) {
            const read = await body.getReader().read();

            body = read.value;
        }

        if (body instanceof Blob) {
            return body.arrayBuffer();
        }

        if (body instanceof FormData) {
            body = multiPartToString(body, this[___response].headers);
        }

        if (typeof body === "string") {
            body = new TextEncoder().encode(body);
        }

        return new Promise((resolve) => resolve(body));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/blob
    async blob() {
        if (this[___response].bodyUsed) {
            throw new TypeError("Failed to execute 'blob': body stream already read");
        }

        if (this[___response].type === "opaque") {
            return new Promise((resolve) => {
                resolve(new Blob([], { type: "" }));
            });
        }

        return this.arrayBuffer().then((buffer) => {
            let type = "";

            const headers = this[___response].headers;

            if (this[___response].body instanceof Blob) {
                type = this[___response].body.type;
            }

            return new Blob([buffer], {
                type: type || headers?.get("content-type") || "",
            });
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/formData
    async formData() {
        if (this[___response].bodyUsed) {
            throw new TypeError("Failed to execute 'formData': body stream already read");
        }

        if (this[___response].body instanceof Blob || this[___response].body instanceof ArrayBuffer) {
            throw new TypeError("Failed to fetch");
        }

        if (this[___response].body instanceof FormData) {
            return new Promise((resolve) => {
                resolve(this[___response].body);
            });
        }

        return this.text().then((text) => toFormData(this[___response].headers, text));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/json
    async json() {
        if (this[___response].bodyUsed) {
            throw new TypeError("Failed to execute 'json': body stream already read");
        }

        return this.text().then((text) => JSON.parse(text));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/text
    async text() {
        if (this[___response].bodyUsed) {
            throw new TypeError("Failed to execute 'text': body stream already read");
        }

        this[___response].bodyUsed = true;

        if (!this[___response].body) {
            return "";
        }

        let body = this[___response].body;

        if (body instanceof ReadableStream) {
            const read = await body.getReader().read();

            body = read.value;
        }

        if (body instanceof Blob) {
            body = await body.arrayBuffer();
        }

        if (body instanceof FormData) {
            return new Promise((resolve) => resolve(multiPartToString(body, this[___response].headers)));
        }

        return new Promise((resolve) => {
            resolve(new TextDecoder().decode(body));
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/headers
    get headers() {
        return this[___response].headers;
    }
    // read-only
    set headers(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/ok
    get ok() {
        return this[___response].ok;
    }
    // read-only
    set ok(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected
    get redirected() {
        return this[___response].redirected;
    }
    // read-only
    set redirected(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/status
    get status() {
        return this[___response].status;
    }
    // read-only
    set status(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText
    get statusText() {
        return this[___response].statusText;
    }
    // read-only
    set statusText(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/type
    get type() {
        return this[___response].type;
    }
    // read-only
    set type(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/url
    get url() {
        return this[___response].url;
    }
    // read-only
    set url(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/clone
    clone() {
        if (this[___response].bodyUsed) {
            throw new TypeError("Failed to execute 'clone' on 'Response': Response body is already use");
        }

        return new Response(this[___response].body, {
            headers: this[___response].headers,
            status: this[___response].status,
            statusText: this[___response].statusText,
            url: this[___response].url,
        });
    }

    toString() {
        return "[object Response]";
    }

    get [Symbol.toStringTag]() {
        return "Response";
    }
}

globalThis.Response = Response;

// @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
const statusTextList = {
    100: "Continue",
    101: "Switching Protocols",
    102: "Processing",
    103: "Early Hints",
    200: "OK",
    201: "Created",
    202: "Accepted",
    203: "Non-Authoritative Information",
    205: "Reset Content",
    206: "Partial Content",
    207: "Multi-Status",
    208: "Already reported",
    226: "IM Used",
    300: "Multiple Choices",
    301: "Moved Permanently",
    302: "Found",
    303: "See Other",
    304: "Not Modified",
    305: "Use Proxy",
    306: "unused",
    307: "Temporary Redirect",
    308: "Permanent Redirect",
    400: "Bad Request",
    401: "Unauthorized",
    402: "Payment Required",
    403: "Forbidden",
    404: "Not Found",
    405: "Method Not Allowed",
    406: "Not Acceptable",
    407: "Proxy Authentication Required",
    408: "Request Timeout",
    409: "Conflict",
    410: "Gone",
    411: "Length Required",
    412: "Precondition Failed",
    413: "Payload Too Large",
    414: "URI Too Long",
    415: "Unsupported Media Type",
    416: "Range Not Satisfiable",
    417: "Expectation Failed",
    418: "I'm a teapot",
    421: "Misdirected Request",
    422: "Unprocessable Entity",
    423: "Locked",
    424: "Failed Dependency",
    425: "Too Early",
    426: "Upgrade Required",
    428: "Precondition Required",
    429: "Too Many Requests",
    431: "Request Header Fields Too Large",
    451: "Unavailable For Legal Reasons",
    500: "Internal Server Error",
    501: "Not Implemented",
    502: "Bad Gateway",
    503: "Service Unavailable",
    504: "Gateway Timeout",
    505: "Http Version Not Supported",
    506: "Variant Also Negotiates",
    507: "Insufficient Storage",
    508: "Loop Detected",
    510: "Not Extended",
    511: "Network Authentication Required",
};

// NOTE: same in request.js
function toFormData(headers, body) {
    const formData = new FormData();

    if (!body) formData;

    const contentType = headers.get("content-type");

    if (/multipart\/form-data/.test(contentType)) {
        const boundary = getBoundary(contentType);

        return boundary ? processMultipart(body, boundary) : formData;
    }

    if (/application\/x-www-form-urlencoded/.test(contentType)) {
        for (const bytes of body.trim().split("&")) {
            if (bytes) {
                const split = bytes.split("=");
                const name = split.shift().replace(/\+/g, " ");
                const value = split.join("=").replace(/\+/g, " ");
                formData.append(decodeURIComponent(name), decodeURIComponent(value));
            }
        }

        return formData;
    }

    throw new TypeError("Failed to fetch");
}

// NOTE: same in request.js
function processMultipart(body, boundary) {
    const formData = new FormData();
    const chunks = body.split(boundary);

    for (const chunk of chunks) {
        let name = "";
        let filename = "";
        let type = "";
        let content = [];
        let isContentStarted = false;

        const lines = chunk.split(/\r?\n/);

        for (let i = 1; i < lines.length; i++) {
            const line = lines[i];
            if (/content-disposition/i.test(line)) {
                const nameMatch = line.match(/\sname\=\"(.*?)\"/);
                const filenameMatch = line.match(/\sfilename\=\"(.*?)\"/);
                name = nameMatch ? nameMatch[1].replace("[]", "") : "";
                filename = filenameMatch ? filenameMatch[1] : "";
            } else if (/content-type/i.test(line)) {
                type = line.match(/content-type:\s*(.*)/i)?.[1] || "";
            } else if (isContentStarted) {
                if (line === "--") continue;
                content.push(line);
            } else if (!line.trim()) {
                isContentStarted = true;
            }
        }

        if (name) {
            content = content.join("\n");
            if (filename && type) {
                const encode = new Uint8Array(Buffer.from(content, "base64"));
                const blob = new Blob([encode], { type });
                formData.append(name, blob, filename);
            } else {
                formData.append(name, content);
            }
        }
    }

    return formData;
}

// NOTE: same in request.js
function getBoundary(contentType) {
    if (!contentType) return "";

    const boundary = contentType.split(";").find((item) => item.includes("boundary"));

    return boundary ? boundary.split("=")[1] : "";
}

// NOTE: same in request.js
function multiPartToString(formData, headers) {
    const crlf = "\r\n";
    const boundary = createBoundary();
    const body = [];

    for (const [name, value] of formData.entries()) {
        body.push(`--${boundary}\r\n`);

        if (value instanceof Blob) {
            body.push(
                `Content-Disposition: form-data; name="${name}"; filename="${value.name}"${crlf}`,
                `Content-Type: ${value.type || "application/octet-stream"}${crlf}${crlf}`,
                value,
                crlf,
            );
        } else {
            body.push(`Content-Disposition: form-data; name="${name}"${crlf}${crlf}${value}${crlf}`);
        }
    }

    body.push(`--${boundary}--`);

    headers.set("content-type", `multipart/form-data; boundary=${boundary}`);

    return body.join("\n");
}

// NOTE: same in request.js
// CREDIT: https://github.com/octet-stream/form-data-encoder/blob/1d08012068ea1088e725630fcbebf7ce98b6dbb4/src/util/createBoundary.ts#L18
function createBoundary() {
    let size = 16;
    let res = "";

    const alphabet = "abcdefghijklmnopqrstuvwxyz0123456789";

    while (size--) {
        res += alphabet[(Math.random() * alphabet.length) << 0];
    }

    return `--------------------------${res}`;
}
