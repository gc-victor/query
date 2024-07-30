const okFunction = `globalThis.___handleRequest = async () => {
    return new Response("OK", { status: 200 });
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(okFunction)).toString()}]`);
