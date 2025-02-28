export async function handleRequest(req: Request) {
    const url = process.env.QUERY_APP_ENV === "development" ? "http://localhost:3000" : "https://qery.io";

    return Response.redirect(`${url}/docs/introduction.html`, 301);
}
