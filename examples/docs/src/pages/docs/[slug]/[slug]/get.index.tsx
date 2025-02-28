export async function handleRequest(req: Request) {
    const url = new URL(req.url);
    const subdir = url.pathname.split("/").slice(0, -1).pop();
    const slug = url.pathname.split("/").pop();

    const db = new Database("query_asset.sql");
    const result: { data: AllowSharedBufferSource }[] = db.query_cache(
        "SELECT data FROM asset WHERE name = $1",
        [`dist/docs/${subdir}/${slug}`],
        10000,
    );

    const styles_result = db.query("SELECT name_hashed FROM asset WHERE name = ?", ["dist/docs/styles.css"]) as {
        name_hashed: string;
    }[];
    const styles = `/_/asset/${styles_result[0].name_hashed}`;

    if (result.length === 0) {
        const result404 = db.query("SELECT data FROM asset WHERE name = 'dist/docs/404.html'");

        if (result404.length === 0) {
            return new Response("Not Found", { status: 404, headers: { "Content-Type": "text/plain" } });
        }

        const html = new TextDecoder().decode((result404[0] as { data: AllowSharedBufferSource }).data);

        return new Response(html.replace("__STYLES_CSS__", styles).replace("__BASE_URL__", url.origin), {
            status: 404,
            headers: { "Content-Type": "text/html; charset=utf-8", "Query-Cache-Control": "max-age=3600000" },
        });
    }

    const html = new TextDecoder().decode((result[0] as { data: AllowSharedBufferSource }).data);

    const headers = {
        "Content-Type": "text/html; charset=utf-8",
    } as Record<string, string>;

    if (process.env.QUERY_APP_ENV === "development") {
        headers["Cache-Control"] = "no-cache";
    } else {
        headers["Cache-Control"] = "max-age=3600";
        headers["Query-Cache-Control"] = "max-age=3600000";
    }

    return new Response(html.replace("__STYLES_CSS__", styles).replace("__BASE_URL__", url.origin), { status: 200, headers });
}
