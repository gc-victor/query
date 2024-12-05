import { Database } from "query:database";
import { HotReload } from "@/pages/hot-reload/hot-reload";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const db = new Database("query_asset.sql");
    const result = db.query("SELECT name_hashed FROM asset WHERE name = ?", ["dist/styles.css"]) as {
        name_hashed: string;
    }[];
    const styles = `/_/asset/${result[0].name_hashed}`;

    return new Response(
        `<!DOCTYPE html>${(
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <meta http-equiv="Content-Type" content="text/html" />
                    <title>No Dynamic</title>
                    <link rel="stylesheet" href={styles} />
                </head>
                <body class="bg-slate-100 text-slate-900">
                    <div class="container mx-auto pt-12 p-4">
                        <h1 class="text-4xl font-cal mb-4 text-slate-700">No Dynamic Route</h1>
                        <p class="text-lg mb-4">This page isn't a dynamic route.</p>
                        <ul class="flex gap-4 mb-4">
                            <li>
                                <a href="/" class="text-blue-600 hover:text-blue-800 underline">
                                    Home
                                </a>
                            </li>
                            <li>
                                <a href="/dynamic" class="text-blue-600 hover:text-blue-800 underline">
                                    Dynamic
                                </a>
                            </li>
                        </ul>
                        <HotReload href={url.href} />
                    </div>
                </body>
            </html>
        )}`,
        {
            status: 200,
            headers: {
                "Content-Type": "text/html; charset=utf-8",
            },
        },
    );
}
