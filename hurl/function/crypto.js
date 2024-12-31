const cryptoFunction = `
import { getRandomValues } from "node:crypto";

globalThis.___handleRequest = async () => {
    try {
        const subtle = await globalThis.crypto.subtle;
        const key = await subtle.generateKey(
            {
                name: "AES-CBC",
                length: 256,
            },
            true,
            ["encrypt", "decrypt"],
        );

        const data = new TextEncoder().encode("OK");
        const iv = getRandomValues(new Uint8Array(16));

        const encrypted = await subtle.encrypt(
            {
                name: "AES-CBC",
                iv,
            },
            key,
            data,
        );

        const decrypted = await subtle.decrypt(
            {
                name: "AES-CBC",
                iv,
            },
            key,
            encrypted,
        );

        const decryptedText = new TextDecoder().decode(decrypted);

        return new Response(decryptedText, { status: 200 });
    } catch (e) {
        console.error(e.message + "\\n" + (e.stack || ""));
        return new Response(e.message + "\\n" + (e.stack || ""), { status: 500 });
    }
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(cryptoFunction)).toString()}]`);
