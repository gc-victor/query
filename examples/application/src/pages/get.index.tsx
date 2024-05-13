import { PAGE_POST_PATH } from "@/config/shared/post.constants";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    return Response.redirect(url.origin + PAGE_POST_PATH);
}
