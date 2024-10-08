// https://marketplace.visualstudio.com/items?itemName=Tobermory.es6-string-html
const js = String.raw;

export async function handleRequest(_: Request) {
    return new Response(
        js`
        <!DOCTYPE html>
        <html>
            <head>
                <title>Minimal</title>
                <script src="/_/asset/public/js/hot-reload.js"></script>
            </head>
            <body>
                <h1>Minimal</h1>
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
