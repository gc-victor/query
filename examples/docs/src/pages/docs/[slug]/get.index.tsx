import { withHtmlRequestErrorHandler } from "@/lib/server/with-html-request-error-handler";
import { DocsPage } from "@/pages/docs/docs";
import { handleError } from "@/pages/docs/handle-error";
import type { DocumentationPage, Toc } from "@/pages/docs/types";
import { getAssetData } from "@/pages/lib/asset-data";
import { htmlResponse as response } from "@/pages/lib/html-response";

async function handleDocsRequest(req: Request) {
    const url = new URL(req.url);
    const slug = url.pathname.split("/").pop();

    const toc = getAssetData<Toc>("dist/docs/toc.json");
    const page = getAssetData<DocumentationPage>(`dist/docs/${slug?.replace(/\.html$/, "")}.json`);

    return response(<DocsPage page={page} url={url} toc={toc} />);
}

export const handleRequest = withHtmlRequestErrorHandler(handleDocsRequest, handleError());
