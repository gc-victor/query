import { Counter } from "@/pages/counter";
// import { HotReload } from "@/pages/hot-reload/hot-reload";
import { getAssetPath } from "@/pages/lib/get-asset-path";
import { render } from "@/pages/lib/render";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const stylesPath = getAssetPath("dist/styles.css");
    const islandCounterPath = getAssetPath("dist/counter.island.js");

    const db = new Database("counter.sql");
    const [counter] = db.query("SELECT value FROM counter WHERE id = 1") as {
        value: number;
    }[];
    const initialValue = counter.value;

    try {
        return new Response(
            render(
                <>
                    <head>
                        <meta charset="UTF-8" />
                        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                        <meta http-equiv="Content-Type" content="text/html" />
                        <title>Counter</title>
                        <link rel="stylesheet" href={stylesPath} />
                        <script src={islandCounterPath} type="module" />
                    </head>
                    <body>
                        <div className="flex flex-col items-center p-8 justify-center h-screen bg-slate-100 dark:bg-slate-900">
                            <div className="bg-white dark:bg-slate-800 p-8 rounded-lg shadow-md w-full max-w-md">
                                <form className="text-center">
                                    <h2 className="text-2xl font-bold mb-4 text-slate-800 dark:text-slate-200">Counter Island</h2>
                                    <div id="counter">
                                        <counter-island>
                                            <Counter count={initialValue} />
                                        </counter-island>
                                    </div>
                                </form>
                            </div>
                        </div>
                        {/* <HotReload href={url.href} /> */}
                    </body>
                </>,
            ),
            {
                headers: {
                    "Content-Type": "text/html; charset=utf-8",
                },
            },
        );
    } catch (e: unknown) {
        console.error((e as string).toString());
        return new Response(e as string, { status: 500 });
    }
}
