import { Database } from "query:database";
import { NotFoundError } from "@/pages/lib/types";

interface PageData {
    path: string;
    updated_at: number;
}

export async function handleRequest(req: Request): Promise<Response> {
    try {
        const url = new URL(req.url);

        const db = new Database("query_asset.sql");
        const pages = db.query(
            "SELECT json_extract (data, '$.path') AS path, updated_at FROM asset WHERE name LIKE 'dist/docs/%.json'",
        ) as PageData[];

        let xml = '<?xml version="1.0" encoding="UTF-8"?>\n';
        xml += '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';

        for (const page of pages) {
            if (!page.path) continue;
            xml += "  <url>\n";
            xml += `    <loc>${url.origin}${page.path.replace("./", "/docs/")}</loc>\n`;
            xml += `    <lastmod>${new Date(page.updated_at * 1000).toISOString().split("T")[0]}</lastmod>\n`;
            xml += "  </url>\n";
        }

        xml += "</urlset>";

        return new Response(xml, {
            status: 200,
            headers: {
                "Content-Type": "application/xml",
                "Cache-Control": "public, max-age=86400", // Cache for 1 day
            },
        });
    } catch (error) {
        if (error instanceof NotFoundError) {
            return new Response("Pages asset not found", {
                status: 404,
                headers: { "Content-Type": "text/plain" },
            });
        }

        console.error("Error generating sitemap:", error);

        return new Response("Error generating sitemap", {
            status: 500,
            headers: { "Content-Type": "text/plain" },
        });
    }
}
