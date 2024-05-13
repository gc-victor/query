import { parse } from "valibot";

import { PostCreateValidation } from "@/api/post/post.validation";
import { QUERY_API_QUERY } from "@/config/server/server.constants";
import { POST_DATABASE } from "@/config/shared/post.constants";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";
import { slugify } from "@/lib/server/slugify";
import { tokenService, validateToken } from "@/lib/server/token";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        validateToken("post", req);

        const token = await tokenService.load("post", req);

        const post = await req.json();
        const title = post.title;
        const content = post.content;
        const slug = post.slug || `/${slugify(title as string)}`;
        const image_url = post.image_url;

        parse(PostCreateValidation, { title, content, slug, image_url });

        const query = "INSERT INTO post (title, content, slug, image_url) VALUES (:title, :content, :slug, :image_url);";
        const params = {
            ":title": title,
            ":content": content,
            ":slug": slug,
            ":image_url": image_url,
        };

        const response = await fetcher(QUERY_API_QUERY, {
            method: Method.POST,
            body: JSON.stringify({ db_name: POST_DATABASE, query, params }),
            headers: {
                [AUTHORIZATION_REQUEST]: `Bearer ${token.query_token}`,
                [CONTENT_TYPE_REQUEST]: "application/json",
            },
        });

        return ok(JSON.stringify(response.json));
    } catch (e) {
        return handleRequestError(e as Error);
    }
}
