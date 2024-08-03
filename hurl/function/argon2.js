const okFunction = `
import { hash, verify } from "query:argon2";

globalThis.___handleRequest = async () => {
    let hashed = hash("password");
    let verified = verify("password", hashed);

    return new Response(verified ? "OK" : "Unauthorized", { status: verified ? 200 : 401 });
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(okFunction)).toString()}]`);
