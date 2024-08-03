const setTimeoutFunction = `globalThis.___handleRequest = async () => {
    setTimeout(() => {
        console.log("Set Timeout 250!!!")
    }, 250);

    return new Response("OK", { status: 200 });
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(setTimeoutFunction)).toString()}]`);
