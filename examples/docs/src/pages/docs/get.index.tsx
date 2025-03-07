export async function handleRequest(req: Request) {
    const url = new URL(req.url)
        
    return Response.redirect(`${url.origin}/docs/introduction.html`, 301);
}