import { test, describe, expect } from "query:test";
import { createHash, createHmac, randomBytes, randomInt, randomUUID, randomFillSync, randomFill, getRandomValues } from "node:crypto";

describe("Basic Crypto Functions", () => {
    test("createHash - SHA256", () => {
        const hash = createHash("sha256");
        hash.update("test");
        expect(hash.digest("hex")).toBe("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    });

    test("createHmac", () => {
        const hmac = createHmac("sha256", "secret");
        hmac.update("test");
        expect(hmac.digest("hex")).toBe("0329a06b62cd16b33eb6792be8c60b158d89a2ee3a876fce9a881ebb488c0914");
    });
});

describe("Random Generation Functions", () => {
    test("randomBytes - synchronous", () => {
        const bytes = randomBytes(16);
        expect(bytes.length).toBe(16);
    });

    test("randomInt - within range", () => {
        const num = randomInt(1, 10);
        expect(num >= 1).toBeTruthy();
        expect(num < 10).toBeTruthy();
    });

    test("randomUUID", () => {
        const uuid = randomUUID();
        expect(uuid).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i);
    });

    test("randomFillSync", () => {
        const buffer = new Uint8Array(16);
        randomFillSync(buffer);
        expect(buffer).not.toEqual(new Uint8Array(16));
    });

    test("randomFill - async", async () => {
        const buffer = new Uint8Array(16);
        await new Promise((resolve, reject) => {
            randomFill(buffer, (err, buf) => {
                if (err) reject(err);
                resolve(buf);
            });
        });
        expect(buffer).not.toEqual(new Uint8Array(16));

        await new Promise((resolve, reject) => {
            randomFill(buffer, 3, 2, (err, buf) => {
                if (err) reject(err);
                resolve(buf);
            });
        });
        expect(buffer).not.toEqual(new Uint8Array(16));
    });

    test("getRandomValues", () => {
        const array = new Uint8Array(16);
        getRandomValues(array);
        expect(array).not.toEqual(new Uint8Array(16));
    });
});

describe("Crypto Subtle API", () => {
    test("digest - SHA-256", async () => {
        const data = new TextEncoder().encode("test");
        const hash = await crypto.subtle.digest("SHA-256", data);
        const hashArray = Array.from(new Uint8Array(hash));
        const hashHex = hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");
        expect(hashHex).toBe("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    });

    test("generateKey - AES", async () => {
        const key = await crypto.subtle.generateKey(
            {
                name: "AES-CBC",
                length: 256,
            },
            true,
            ["encrypt", "decrypt"],
        );
        expect(key).toBeTruthy();
    });

    test("encrypt and decrypt - AES-CBC", async () => {
        const key = await crypto.subtle.generateKey(
            {
                name: "AES-CBC",
                length: 256,
            },
            true,
            ["encrypt", "decrypt"],
        );

        const data = new TextEncoder().encode("test message");
        const iv = crypto.getRandomValues(new Uint8Array(16));

        const encrypted = await crypto.subtle.encrypt(
            {
                name: "AES-CBC",
                iv,
            },
            key,
            data,
        );

        const decrypted = await crypto.subtle.decrypt(
            {
                name: "AES-CBC",
                iv,
            },
            key,
            encrypted,
        );

        const decryptedText = new TextDecoder().decode(decrypted);
        expect(decryptedText).toBe("test message");
    });

    test("sign and verify - HMAC", async () => {
        const key = await crypto.subtle.generateKey(
            {
                name: "HMAC",
                hash: "SHA-256",
            },
            true,
            ["sign", "verify"],
        );

        const data = new TextEncoder().encode("test message");
        const signature = await crypto.subtle.sign("HMAC", key, data);

        const isValid = await crypto.subtle.verify("HMAC", key, signature, data);

        expect(isValid).toBeTruthy();
    });

    test("exportKey - HMAC", async () => {
        const key = await crypto.subtle.generateKey(
            {
                name: "HMAC",
                hash: "SHA-256",
            },
            true,
            ["sign", "verify"],
        );

        const exportedKey = await crypto.subtle.exportKey("raw", key);
        expect(exportedKey).toBeInstanceOf(ArrayBuffer);
        expect(exportedKey.byteLength).toBe(32); // HMAC-SHA256 key is 32 bytes
    });
});

describe("Error Cases", () => {
    test("createHash - invalid algorithm", () => {
        expect(() => crypto.createHash("invalid")).toThrow();
    });

    test("createHmac - invalid key", () => {
        expect(() => crypto.createHmac("sha256", null)).toThrow();
    });

    test("randomInt - invalid range", () => {
        expect(() => crypto.randomInt(-1, 1)).toThrow();
        expect(() => crypto.randomInt(2, 1)).toThrow();
    });

    test("getRandomValues - invalid array", () => {
        expect(() => crypto.getRandomValues("not an array")).toThrow();
    });

    test("subtle.digest - invalid algorithm", async () => {
        const data = new TextEncoder().encode("test");
        try {
            await crypto.subtle.digest("INVALID-HASH", data);
            expect(false).toBeTruthy();
        } catch (err) {
            expect(err).toBeTruthy();
        }
    });
});
