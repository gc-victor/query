const cacheControlFunction = `globalThis.___handleRequest = async () => {
    return new Response("Hurl!", {
		status: 200,
		headers: {
			"content-type": "text/plain;charset=UTF-8",
            "Query-Cache-Control": "public, max-age=500", // Cache for 0.5 seconds
		},
	});
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(funFunFunction)).toString()}]`);
