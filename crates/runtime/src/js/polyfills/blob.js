const ___parts = Symbol();
const ___endings = Symbol();

// 64 KiB (same size chrome slice theirs blob into Uint8array's)
const POOL_SIZE = 65536;

// @see: https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob
// @see: https://github.com/node-fetch/fetch-blob/blob/57e4daef36081936581d14509b6cc618d87ab9e2/index.js
class Blob {
    constructor(blobParts, options) {
        this[___parts] = [];
        this[___endings] = "transparent";

        this.size = 0;
        this.type = "";

        if (blobParts !== undefined) {
            if (typeof blobParts !== "object" || blobParts === null) {
                throw new TypeError("Failed to construct 'Blob': The provided value cannot be converted to a sequence.");
            }

            if (typeof blobParts[Symbol.iterator] !== "function") {
                throw new TypeError("Failed to construct 'Blob': The object must have a callable @@iterator property.");
            }

            for (const element of blobParts) {
                let part;

                if (ArrayBuffer.isView(element)) {
                    part = new Uint8Array(element.buffer.slice(element.byteOffset, element.byteOffset + element.byteLength));
                } else if (element instanceof ArrayBuffer) {
                    part = new Uint8Array(element.slice(0));
                } else if (element instanceof Blob) {
                    part = element;
                } else {
                    part = new TextEncoder().encode(`${element}`);
                }

                const size = ArrayBuffer.isView(part) ? part.byteLength : part.size;

                // Avoid pushing empty parts into the array to better GC them
                if (size) {
                    this.size += size;
                    this[___parts].push(part);
                }
            }
        }

        if (options !== undefined && typeof options !== "object" && typeof options !== "function") {
            throw new TypeError("Failed to construct 'Blob': parameter 2 cannot convert to dictionary.");
        }

        const normalizedOptions = options === null || options === undefined ? {} : options;

        this[___endings] = `${normalizedOptions.endings === undefined ? "transparent" : normalizedOptions.endings}`;
        const type = normalizedOptions.type === undefined ? "" : String(normalizedOptions.type);
        this.type = /^[\x20-\x7E]*$/.test(type) ? type : "";
    }

    async text() {
        // More optimized than using this.arrayBuffer()
        // that requires twice as much ram
        const decoder = new TextDecoder();

        let str = "";

        for await (const part of toIterator(this[___parts], false)) {
            str += decoder.decode(part);
        }
        // Remaining
        str += decoder.decode();

        return str;
    }

    async arrayBuffer() {
        const data = new Uint8Array(this.size);
        let offset = 0;
        for await (const chunk of toIterator(this[___parts], false)) {
            data.set(chunk, offset);
            offset += chunk.length;
        }

        return data.buffer;
    }

    stream() {
        const it = toIterator(this[___parts], true);

        return new ReadableStream({
            type: "bytes",
            async pull(ctrl) {
                const chunk = await it.next();
                chunk.done ? ctrl.close() : ctrl.enqueue(chunk.value);
            },

            async cancel() {
                await it.return();
            },
        });
    }

    slice(start = 0, end = this.size, type = "") {
        const { size } = this;

        let relativeStart = start < 0 ? Math.max(size + start, 0) : Math.min(start, size);
        let relativeEnd = end < 0 ? Math.max(size + end, 0) : Math.min(end, size);

        const span = Math.max(relativeEnd - relativeStart, 0);
        const parts = this[___parts];
        const blobParts = [];
        let added = 0;

        for (const part of parts) {
            // don't add the overflow to new blobParts
            if (added >= span) {
                break;
            }

            const size = ArrayBuffer.isView(part) ? part.byteLength : part.size;
            if (relativeStart && size <= relativeStart) {
                // Skip the beginning and change the relative
                // start & end position as we skip the unwanted parts
                relativeStart -= size;
                relativeEnd -= size;
            } else {
                let chunk;
                if (ArrayBuffer.isView(part)) {
                    chunk = part.subarray(relativeStart, Math.min(size, relativeEnd));
                    added += chunk.byteLength;
                } else {
                    chunk = part.slice(relativeStart, Math.min(size, relativeEnd));
                    added += chunk.size;
                }
                relativeEnd -= size;
                blobParts.push(chunk);
                relativeStart = 0; // All next sequential parts should start at 0
            }
        }

        const blob = new Blob([], { type: String(type).toLowerCase() });
        blob.size = span;
        blob[___parts] = blobParts;

        return blob;
    }
}

globalThis.Blob = Blob;

async function* toIterator(parts, clone = true) {
    for (const part of parts) {
        if ("stream" in part) {
            yield* part.stream();
        } else if (ArrayBuffer.isView(part)) {
            if (clone) {
                let position = part.byteOffset;
                const end = part.byteOffset + part.byteLength;
                while (position !== end) {
                    const size = Math.min(end - position, POOL_SIZE);
                    const chunk = part.buffer.slice(position, position + size);
                    position += chunk.byteLength;
                    yield new Uint8Array(chunk);
                }
            } else {
                yield part;
            }
            /* c8 ignore next 10 */
        } else {
            // For blobs that have arrayBuffer but no stream method (nodes buffer.Blob)
            let position = 0;
            const b = part;
            while (position !== b.size) {
                const chunk = b.slice(position, Math.min(b.size, position + POOL_SIZE));
                const buffer = await chunk.arrayBuffer();
                position += buffer.byteLength;
                yield new Uint8Array(buffer);
            }
        }
    }
}
