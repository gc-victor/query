import { htmlResponse as response } from "@/pages/lib/html-response";
import { assetPath } from "@/pages/lib/asset-path";
import type { DocumentationPage, Navigation, Toc } from "@/pages/docs/types";
import { DocumentTemplate } from "@/pages/docs/components/document";
import { Scripts } from "@/pages/docs/components/scripts";
import { Icons } from "@/pages/docs/components/icons";
import { NotFoundResponse } from "@/pages/docs/404";
import { InternalServerErrorResponse } from "@/pages/docs/500";
import { getAssetData } from "@/pages/lib/asset-data";
import { NotFoundError } from "@/pages/lib/types";
import { withHtmlRequestErrorHandler } from "@/lib/server/with-html-request-error-handler";

async function handleDocsRequest(req: Request) {
    const url = new URL(req.url);
    const slug = url.pathname.split("/").pop();

    const toc = getAssetData<Toc>("dist/docs/toc.json");
    const page = getAssetData<DocumentationPage>(`dist/docs/${slug?.replace(/\.html$/, "")}.json`);

    return response(
        <html lang="en">
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
        </html>,
    );
}

export const handleRequest = withHtmlRequestErrorHandler(handleDocsRequest, (req, error) => {
    const url = new URL(req.url);
    const toc = getAssetData<Toc>("dist/docs/toc.json");

    if (error instanceof NotFoundError) {
        return NotFoundResponse({ origin: url.origin, toc });
    }

    return InternalServerErrorResponse({ origin: url.origin, toc });
});
