import { assetPath } from "@/pages/lib/asset-path";
import { DocumentTemplate } from "./components/document";
import { Icons } from "./components/icons";
import { Scripts } from "./components/scripts";
import type { DocumentationPage, Toc, Navigation } from "./types";

export function DocsPage({ page, url, toc }: {page: DocumentationPage, url: URL, toc: Toc}): JSX.Element {
    return <html lang="en">
        <head>
            <meta charSet="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta httpEquiv="Content-Type" content="text/html" />
            <title>{page.title} - Documentation</title>
            <meta name="description" content={page.description} />
            <base href={`${url.origin}/docs/`} />
            <link rel="stylesheet" href={assetPath("dist/docs/styles.css")} />

            <link rel="apple-touch-icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon.svg" />
            <link rel="icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon.svg" sizes="any" />
            <link rel="mask-icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon-black.svg" />
        </head>

        <body class="flex min-h-full bg-white antialiased dark:bg-slate-900">
            <DocumentTemplate content={page.content} navigation={page.navigation as Navigation} toc={toc} />
            <Icons />
            <Scripts />
        </body>
    </html>;
}
