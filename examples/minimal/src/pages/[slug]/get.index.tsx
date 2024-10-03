// https://marketplace.visualstudio.com/items?itemName=Tobermory.es6-string-html
const js = String.raw;

export async function handleRequest(req: Request) {
    const url = new URL(req.url);
    const href = url.href;
    const slug = href.split("/").pop();
    const title = slug ? slug.charAt(0).toUpperCase() + slug.slice(1) : "";

    return new Response(
        js`
        <!DOCTYPE html>
        <html>
            <head>
                <title>${title}</title>
                <script src="/_/asset/public/js/hot-reload.js"></script>
            </head>
            <body>
                <h1>${title}</h1>
                <p></p>
            </body>
        </html>
    `,
        {
            status: 200,
            headers: {
                "Content-Type": "text/html; charset=utf-8",
            },
        },
    );
}
