const setTimeoutFunction = `globalThis.___handleRequest = async () => {
    setInterval(() => {
        console.log("Set Interval 60000!!!")
    }, 60000);

    return new Response("OK", { status: 200 });
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(setTimeoutFunction)).toString()}]`);
