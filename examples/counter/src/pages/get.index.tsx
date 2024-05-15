import { signal } from "@preact/signals";
import { getNameHashed } from "@/pages/lib/get-bundle-files";
import { render } from "@/pages/lib/render";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Counter } from "@/pages/counter";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const stylesNameHashed = await getNameHashed("dist/styles.css");
    const islandCounterNameHashed = await getNameHashed("dist/counter.island.js");

    const count = signal(3);

    try {
        return new Response(
            render(
                <>
                    <head>
                        <title>Counter</title>
                        <link rel="stylesheet" href={stylesNameHashed} />
                    </head>
                    <body>
                        <div className="flex flex-col items-center justify-center h-screen bg-slate-100 dark:bg-slate-900">
                            <div className="bg-white dark:bg-slate-800 p-8 rounded-lg shadow-md w-full max-w-md">
                                <form className="text-center">
                                    <h2 className="text-2xl font-bold mb-4 text-slate-800 dark:text-slate-200">Counter Island</h2>
                                    <div id="counter">
                                        <Counter count={count} />
                                    </div>
                                    <p className="mt-4 text-slate-500 dark:text-slate-400">
                                        Initial Value: <span id="count">{count}</span>
                                    </p>
                                </form>
                            </div>
                        </div>
                        <HotReload href={url.href} />
                        <script src={islandCounterNameHashed} type="module" />
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
