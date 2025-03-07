import { htmlResponse as response } from "@/pages/lib/html-response";
import { assetPath } from "@/pages/lib/asset-path";
import { DocumentTemplate } from "@/pages/docs/components/document";
import { Scripts } from "@/pages/docs/components/scripts";
import { NotFound } from "@/pages/docs/components/not-found";
import { Icons } from "@/pages/docs/components/icons";
import type { Toc } from "./types";

export function NotFoundResponse({ origin, toc }: { origin: string; toc: Toc }) {
    const notFound = <NotFound />;

    return response(
        <html lang="en">
            <head>
                <meta charSet="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <meta httpEquiv="Content-Type" content="text/html" />
                <title>Not Found - Documentation</title>
                <base href={`${origin}/docs/`} />
                <link rel="stylesheet" href={assetPath("dist/docs/styles.css")} />

                <link rel="apple-touch-icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon.svg" />
                <link rel="icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon.svg" sizes="any" />
                <link rel="mask-icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon-black.svg" />
            </head>
            <body class="flex min-h-full bg-white antialiased dark:bg-slate-900">
                <DocumentTemplate content={notFound} navigation={null} toc={toc} />
                <Icons />
                <Scripts />
            </body>
        </html>,
    );
}
