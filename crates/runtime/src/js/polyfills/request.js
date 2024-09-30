const ___request = Symbol();

/*
 * Request
 *
 * The Request interface of the Fetch API represents a resource request.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/Request
 * @see: https://fetch.spec.whatwg.org/#request-class
 * @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L339
 */
class Request {
    constructor(input, init) {
        this[___request] = {};

        const isRequest = input instanceof Request;
        const options = isRequest ? input : init || {};
        const body = options.body;

        if (/CONNECT|TRACE|TRACK/i.test(options.method)) {
            throw new TypeError(`Failed to construct 'Request': '${options.method}' HTTP method is unsupported`);
        }

        if ((!options.method || /GET|HEAD/i.test(options.method)) && body) {
            throw new TypeError("Body not allowed for GET or HEAD requests");
        }

        this[___request].body = typeof body === "string" ? new TextEncoder().encode(body) : body || null;

        this[___request].cache = options?.cache || "default";
        this[___request].credentials = options?.credentials || "same-origin";
        this[___request].destination = "worker";
        this[___request].integrity = options?.integrity || "";
        this[___request].keepalive = !!options?.keepalive;
        this[___request].method = options?.method?.toUpperCase() || "GET";
        this[___request].mode = options?.mode || "cors";
        this[___request].redirect = options?.redirect || "follow";
        this[___request].referrer = options?.referrer || "";
        this[___request].referrerPolicy = options?.referrerPolicy || "";
        this[___request].signal = options?.signal || null;
        this[___request].url = isRequest ? input.url.toString() : input.toString();

        if (options.headers) {
            const headers = new Headers(isRequest && !init?.headers ? options.headers.getAll() : init.headers);

            if (!headers.has("content-type") && this[___request].body) {
                // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#body
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

            this[___request].headers = headers;
        }

        if (this[___request].method === "GET" || this[___request].method === "HEAD") {
            if (options.cache === "no-store" || options.cache === "no-cache") {
                // Search for a '_' parameter in the query string
                const reParamSearch = /([?&])_=[^&]*/;
                if (reParamSearch.test(this[___request].url)) {
                    // If it already exists then set the value with the current time
                    this[___request].url = this[___request].url.replace(reParamSearch, `$1_=${new Date().getTime()}`);
                } else {
                    // Otherwise add a new '_' parameter to the end with the current time
                    const reQueryString = /\?/;
                    this[___request].url += `${reQueryString.test(this[___request].url) ? "&" : "?"}_=${new Date().getTime()}`;
                }
            }
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/body
    get body() {
        if (this[___request].body === null) return null;
        if (this[___request].body instanceof ReadableStream) return this[___request].body;

        const stream = new TransformStream();
        const writer = stream.writable.getWriter();

        writer.write(this[___request].body);
        writer.close();

        return stream.readable;
    }

    // read-only
    set body(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed
    get bodyUsed() {
        return this[___request].bodyUsed;
    }
    // read-only
    set bodyUsed(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer
    async arrayBuffer() {
        if (this[___request].bodyUsed) {
            throw new TypeError("Failed to execute 'arrayBuffer': body stream already read");
        }

        let body = this[___request].body;

        if (body instanceof ReadableStream) {
            const read = await body.getReader().read();

            body = read.value;
        }

        if (body instanceof Blob) {
            return body.arrayBuffer();
        }

        if (body instanceof FormData) {
            body = multiPartToString(body, this[___request].headers);
        }

        if (typeof body === "string") {
            body = new TextEncoder().encode(body);
        }

        return new Promise((resolve) => resolve(body));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/blob
    async blob() {
        if (this[___request].bodyUsed) {
            throw new TypeError("Failed to execute 'blob': body stream already read");
        }

        if (this[___request].type === "opaque") {
            return new Promise((resolve) => {
                resolve(new Blob([], { type: "" }));
            });
        }

        return this.arrayBuffer().then((buffer) => {
            let type = "";

            const headers = this[___request].headers;

            if (this[___request].body instanceof Blob) {
                type = this[___request].body.type;
            }

            return new Blob([buffer], {
                type: type || headers?.get("content-type") || "",
            });
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/formData
    async formData() {
        if (this[___request].bodyUsed) {
            throw new TypeError("Failed to execute 'formData': body stream already read");
        }

        if (this[___request].body instanceof Blob || this[___request].body instanceof ArrayBuffer) {
            throw new TypeError("Failed to fetch");
        }

        if (this[___request].body instanceof FormData) {
            return new Promise((resolve) => {
                resolve(this[___request].body);
            });
        }

        return this.text().then((text) => toFormData(this[___request].headers, text));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/json
    async json() {
        if (this[___request].bodyUsed) {
            throw new TypeError("Failed to execute 'json': body stream already read");
        }

        return this.text().then((text) => JSON.parse(text));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/text
    async text() {
        if (this[___request].bodyUsed) {
            throw new TypeError("Failed to execute 'text': body stream already read");
        }

        this[___request].bodyUsed = true;

        if (!this[___request].body) {
            return "";
        }

        let body = this[___request].body;

        if (body instanceof ReadableStream) {
            const read = await body.getReader().read();

            body = read.value;
        }

        if (body instanceof Blob) {
            body = await body.arrayBuffer();
        }

        if (body instanceof FormData) {
            return new Promise((resolve) => resolve(multiPartToString(body, this[___request].headers)));
        }

        return new Promise((resolve) => {
            resolve(new TextDecoder().decode(body));
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
    get cache() {
        return this[___request].cache;
    }
    // readonly
    set cache(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials
    get credentials() {
        return this[___request].credentials;
    }
    // readonly
    set credentials(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/destination
    get destination() {
        return this[___request].destination;
    }
    // readonly
    set destination(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/headers
    get headers() {
        return this[___request].headers;
    }
    // readonly
    set headers(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity
    get integrity() {
        return this[___request].integrity;
    }
    // readonly
    set integrity(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#keepalive
    get keepalive() {
        return this[___request].keepalive;
    }
    // readonly
    set keepalive(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/method
    get method() {
        return this[___request].method;
    }
    // readonly
    set method(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    get mode() {
        return this[___request].mode;
    }
    // readonly
    set mode(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect
    get redirect() {
        return this[___request].redirect;
    }
    // readonly
    set redirect(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer
    get referrer() {
        return this[___request].referrer;
    }
    // readonly
    set referrer(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy
    get referrerPolicy() {
        return this[___request].referrerPolicy;
    }
    // readonly
    set referrerPolicy(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#signal
    get signal() {
        return this[___request].signal || new AbortSignal();
    }
    // readonly
    set signal(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/url
    get url() {
        return this[___request].url;
    }
    // readonly
    set url(_) {}

    clone() {
        if (this[___request].bodyUsed) {
            throw new TypeError("Failed to execute 'clone' on 'Request': Request body is already used");
        }

        return new Request(this[___request].url, {
            body: this[___request].body,
            cache: this[___request].cache,
            credentials: this[___request].credentials,
            headers: this[___request].headers,
            integrity: this[___request].integrity,
            keepalive: this[___request].keepalive,
            method: this[___request].method,
            mode: this[___request].mode,
            redirect: this[___request].redirect,
            referrer: this[___request].referrer,
            referrerPolicy: this[___request].referrerPolicy,
            signal: this[___request].signal,
        });
    }

    toString() {
        return "[object Request]";
    }

    get [Symbol.toStringTag]() {
        return "Request";
    }
}

globalThis.Request = Request;

// NOTE: same in response.js
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

// NOTE: same in response.js
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
                name = nameMatch ? nameMatch[1] : "";
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

// NOTE: same in response.js
function getBoundary(contentType) {
    if (!contentType) return "";

    const boundary = contentType.split(";").find((item) => item.includes("boundary"));

    return boundary ? boundary.split("=")[1] : "";
}

// NOTE: same in response.js
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

// NOTE: same in response.js
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
