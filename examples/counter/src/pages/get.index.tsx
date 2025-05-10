import { Counter } from "@/pages/counter";
// import { HotReload } from "@/pages/hot-reload/hot-reload";
import { assetPath } from "@/pages/lib/asset-path";
import { htmlResponse } from "@/pages/lib/server/response";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const stylesPath = assetPath("dist/styles.css");
    const islandCounterPath = assetPath("dist/counter.island.js");

    const db = new Database("counter.sql");
    const [counter] = db.query("SELECT value FROM counter WHERE id = 1") as {
        value: number;
    }[];
    const initialValue = counter.value;

    try {
        return htmlResponse(
            <>
                <head>
                    <meta charSet="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <meta httpEquiv="Content-Type" content="text/html" />
                    <title>Counter</title>
                    <link rel="stylesheet" href={stylesPath} />
                    <script src={islandCounterPath} type="module" />
                </head>
                <body>
                    <div className="flex flex-col items-center p-8 justify-center h-screen bg-slate-100">
                        <div className="bg-white p-8 rounded-lg shadow-md w-full max-w-md">
                            <form className="text-center">
                                <h2 className="text-2xl font-cal mb-4 text-slate-800">Counter</h2>
                                <div id="counter">
                                    <Counter count={initialValue} />
                                </div>
                            </form>
                        </div>
                    </div>
                    {/* <HotReload href={url.href} /> */}
                </body>
            </>,
        );
    } catch (e: unknown) {
        console.error((e as string).toString());
        return new Response(e as string, { status: 500 });
    }
}
