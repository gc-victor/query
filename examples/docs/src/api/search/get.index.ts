import { Database } from "query:database";
import { withRequestErrorHandler } from "@/lib/server/with-request-error-handler";
import { checkRateLimit } from "@/lib/server/rate-limit";
import { IS_DEVELOPMENT } from "@/config";

interface SearchResult {
    title: string;
    path: string;
    snippet: string;
}

interface SearchResponse {
    results: SearchResult[] | [];
}

async function handleSearchRequest(req: Request): Promise<Response> {
    await checkRateLimit(req, {
        maxRequests: 20,
        windowSeconds: 30,
    });

    const url = new URL(req.url);
    const query = url.searchParams.get("q") || "";

    const words = query
        .replace(/[\\"':*^$~()\[\]{}<>!|&+\-]/g, " ")
        .replace(/\s+/g, " ")
        .trim()
        .toLowerCase()
        .split(/\s+/)
        .filter((word) => word.length > 0 && !["or", "and", "not", "near", "match"].includes(word));

    if (words.length === 0) {
        return emptyQueryResponse();
    }

    const searchQuery = words.length === 1 ? `${words[0]} OR ${words[0]}*` : `${words.join(" ")} OR ${words.join(" OR ")}`;
    const db = new Database("query_asset.sql");
    const results = db.query<SearchResult>(
        `SELECT title, path, snippet(docs_search, 2, '<strong>', '</strong>', '', 20) as section FROM docs_search WHERE docs_search MATCH ? ORDER BY rank LIMIT 5`,
        [searchQuery],
    );

    const searchResponse: SearchResponse = {
        results,
    };

    return new Response(JSON.stringify(searchResponse), {
        status: 200,
        headers: IS_DEVELOPMENT
            ? {
                  "Content-Type": "application/json",
              }
            : {
                  "Content-Type": "application/json",
                  "Cache-Control": "public, max-age=300",
                  "Query-Cache-Control": "max-age=300000",
              },
    });
}

function emptyQueryResponse() {
    const emptyResponse: SearchResponse = {
        results: [],
    };

    return new Response(JSON.stringify(emptyResponse), {
        status: 200,
        headers: IS_DEVELOPMENT
            ? {
                  "Content-Type": "application/json",
              }
            : {
                  "Content-Type": "application/json",
                  "Cache-Control": "max-age=3600",
                  "Query-Cache-Control": "max-age=3600000",
              },
    });
}

export const handleRequest = withRequestErrorHandler(handleSearchRequest);
