const pluginFunction = `
import { plugin } from 'query:plugin';

globalThis.___handleRequest = async () => {
    const argon2 = {
        hash: (password) => plugin("plugin_argon2.wasm", "hash", "password", null),
        verify: (password, hash) => plugin("plugin_argon2.wasm", "verify", JSON.stringify({password, hash}), null) == "true"
    };

    const password = "password";
    const hashedPassword = argon2.hash(password);
    const isPasswordValid = argon2.verify(password, hashedPassword);

    console.log("{password, hashedPassword}", JSON.stringify({password, hashedPassword}));
    console.log("hashedPassword", hashedPassword);
    console.log("isPasswordValid", isPasswordValid);

    try {
        return new Response(isPasswordValid ? "OK" : "Error", { status: 200 });
    } catch (error) {
        return new Response(e.message + "\\n" + (e.stack || ""), { status: 500 });
    }
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(pluginFunction)).toString()}]`);
