import { alerts, gpu, storage } from "../features";
import { runTests } from "../test";
import tests from "../vendor/tests.json";

export async function handleRequest(_) {
    try {
        const testString = tests as unknown as string;
        const data = await runTests(JSON.parse(testString), [...alerts, ...gpu, ...storage]);

        return new Response(JSON.stringify(data), {
            headers: {
                "content-type": "application/json",
            },
        });
    } catch (e) {
        console.error(e);

        return new Response(
            JSON.stringify({
                message: e.message,
                stack: e.stack,
            }),
            {
                status: 500,
                headers: {
                    "content-type": "application/json",
                },
            },
        );
    }
}
