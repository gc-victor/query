type RequestHandler = (req: Request) => Promise<Response>;
type HandleRequestError = (req: Request, e: Error) => Response | undefined;

export function withHtmlRequestErrorHandler(handler: RequestHandler, handleRequestError?: HandleRequestError): RequestHandler {
    return async (req: Request): Promise<Response> => {
        try {
            return await handler(req);
        } catch (e) {
            const error = e as Error;

            console.error("Error:", `${error.message}\\n${error.stack || ""}`);

            let errorResponse: Response | undefined;

            if (handleRequestError) {
                try {
                    errorResponse = handleRequestError(req, error);
                } catch (e) {
                    const error = e as Error;

                    console.error("Error in handleRequestError:", `${error.message}\\n${error.stack || ""}`);
                }
            }

            return (
                errorResponse ||
                new Response(`<!DOCTYPE html>${html()}`, {
                    status: 500,
                    headers: {
                        "Content-Type": "text/html; charset=utf-8",
                    },
                })
            );
        }
    };
}

function html() {
    return <html lang="en">
        <head>
            <meta charSet="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta httpEquiv="Content-Type" content="text/html" />
            <title>Internal Server Error - Documentation</title>
            <link rel="stylesheet" href="/_/asset/dist/docs/styles.css" />
            <link rel="apple-touch-icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon.svg" />
            <link rel="icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon.svg" sizes="any" />
            <link rel="mask-icon" type="image/svg" href="/_/asset/public/images/cache/favicon/favicon-black.svg" />
        </head>
        <body>
            <main className="error-page text-wrap max-w-screen-sm overflow-hidden">
                <h1>500</h1>
                <h2>Internal Server Error</h2>
                <p>Sorry, something went wrong. Please try again later.</p>
            </main>
        </body>
    </html>;
}

