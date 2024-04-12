import bcd from "../../../node_modules/@mdn/browser-compat-data/data.json";
import data from "../../../download/data.json";

export async function handleRequest(_) {
    try {
        const testString = JSON.parse(bcd) as unknown as string;
        const data = await runTests(JSON.parse(testString), [...alerts, ...gpu, ...storage]);

        return new Response(JSON.stringify(data), {
            headers: {
                "content-type": "text/html"
            },
        });
    } catch (e) {
        console.error(e);

        return new Response(e.message, { status: 500 });
    }
}