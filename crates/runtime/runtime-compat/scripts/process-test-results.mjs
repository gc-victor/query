import { writeFile } from "node:fs/promises";
import data from "@mdn/browser-compat-data" assert { type: "json" };
import compatData from "../download/data.json" assert { type: "json" };

const support = {};
support.runtime = new Set(getSupportedApis(compatData));

console.log(`Found ${support.runtime.size} supported APIs`);

const union = new Set(
    Object.values(support)
        .flatMap((set) => [...set])
        .sort(),
);

let compat = {};

for (const feature of union) {
    const entry = deepCreate(compat, feature);
    if (!entry.__compat) {
        
        const browserCompat = get(data, `${feature}.__compat`) ?? {};
        entry.__compat = {
            ...browserCompat,
            support: [
                "runtime",
                {
                    version_added: support.runtime.has(feature),
                },
            ],
        };
    }
}

// https://common-min-api.proposal.wintercg.org/#index
const winterCGAPIs = [
    "AbortController",
    "AbortSignal",
    "Blob",
    "ByteLengthQueuingStrategy",
    "CompressionStream",
    "CountQueuingStrategy",
    "Crypto",
    "CryptoKey",
    "DecompressionStream",
    "DOMException",
    "Event",
    "EventTarget",
    "File",
    "FormData",
    "Headers",
    "ReadableByteStreamController",
    "ReadableStream",
    "ReadableStreamBYOBReader",
    "ReadableStreamBYOBRequest",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "Request",
    "Response",
    "SubtleCrypto",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TransformStream",
    "TransformStreamDefaultController",
    "URL",
    "URLSearchParams",
    "WritableStream",
    "WritableStreamDefaultController",
    "atob",
    "btoa",
    "console",
    "crypto",
    "fetch",
    "navigator",
    "performance",
    "queueMicrotask",
    "setTimeout",
    "clearTimeout",
    "setInterval",
    "clearInterval",
    "structuredClone",
];

for (const feature of winterCGAPIs) {
    const browserCompat = get(data, `api.${feature}.__compat`) ?? {};

    if (!compat.api[feature]) {
        compat.api[feature] = {
            __compat: {
                ...browserCompat,
                support: [
                    "runtime",
                    {
                        version_added: false,
                    },
                ],
            },
        };
    }
}

compat = sortObject(compat);

await writeFile(new URL("../result.json", import.meta.url), JSON.stringify(compat));

const api = compat.api;
let markdown = "";

for (const key in api) {
    if (Object.prototype.hasOwnProperty.call(api, key)) {
        const tickOrCross = api[key].__compat.support[1].version_added ? '✓' : '✗';
        markdown += `- [${tickOrCross}] [${key}](${api[key].__compat.mdn_url || api[key].__compat.spec_url})\n`;

        const children = api[key];
        for (const childKey in children) {
            if (Object.prototype.hasOwnProperty.call(children, childKey) && childKey !== '__compat') {
                const tickOrCross = children[childKey].__compat.support[1].version_added ? '✓' : '✗';
                markdown += `  - [${tickOrCross}] [${childKey}](${children[childKey].__compat.mdn_url || children[childKey].__compat.spec_url})\n`;
            }
        }
    }
}

console.log(markdown);

function getSupportedApis(results) {
    const passes = [];
    for (const test of results) {
        if (test.result) {
            passes.push(test.name);
        }
    }
    return passes;
}

function deepCreate(obj, path) {
    const keys = path.split(".");
    let current = obj;
    for (const key of keys) {
        if (!current[key]) {
            current[key] = {};
        }
        current = current[key];
    }
    return current;
}

function get(obj, path) {
    const keys = path.split(".");
    let current = obj;
    for (const key of keys) {
        if (!current[key]) {
            return undefined;
        }
        current = current[key];
    }
    return current;
}

function sortObject(obj) {
    if (typeof obj !== 'object' || obj === null) {
        return obj;
    }

    if (Array.isArray(obj)) {
        return obj.map(sortObject);
    }

    return Object.keys(obj).sort().reduce((result, key) => {
        result[key] = sortObject(obj[key]);
        return result;
    }, {});
}