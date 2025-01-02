import { alerts, gpu, storage } from "../features";
import { runTests } from "../test";
import tests from "../vendor/tests.json";

export async function handleRequest(_) {
    try {
        const testString = tests as unknown as string;
        const data = await runTests(JSON.parse(testString), [...alerts, ...gpu, ...storage]);

        return new Response(JSON.stringify(data), {
            headers: {
                "content-disposition": 'attachment; filename="data.json"',
                "content-type": "application/json"
            },
        });
    } catch (e) {
        console.error(e);

        return new Response(e.message, { status: 500 });
    }
}