import { parse } from "valibot";

import { QUERY_API_ASSET_BUILDER, QUERY_API_QUERY } from "@/config/server/server.constants";
import { POST_DATABASE } from "@/config/shared/post.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";
import { slugify } from "@/lib/server/slugify";
import { url } from "@/lib/server/url";

import { PostUpdateValidation } from "./post.validation";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        const session = await getAdminUserSession(req);
        const isExpired = await adminUserSession.isExpired(session);

        if (isExpired) {
            await adminUserSession.refresh(session);
        }

        const { token } = await adminUserSession.load(session);

        const formData = await req.formData();
        const title = formData.get("title");
        const content = formData.get("content");
        const slug = formData.get("slug") || `/${slugify(title as string)}`;
        const uuid = formData.get("uuid");
        const image = formData.get("image") as File;
        // NOTE: Workaround to send binary data as it fails in Query as it isn't a valid UTF-8 string
        const image_url = image.name ? `post/cache/${image.name}` : formData.get("image_url");

        parse(PostUpdateValidation, { uuid, title, content, slug, image_url });

        const query = "UPDATE post SET title = :title, content = :content, slug = :slug, image_url = :image_url WHERE uuid = :uuid;";
        const params = {
            ":uuid": uuid,
            ":content": content,
            ":title": title,
            ":slug": slug,
            ":image_url": image_url,
        };

        const response = await fetcher(QUERY_API_QUERY, {
            method: Method.POST,
            body: JSON.stringify({ db_name: POST_DATABASE, query, params }),
            headers: {
                [AUTHORIZATION_REQUEST]: `Bearer ${token}`,
                [CONTENT_TYPE_REQUEST]: "application/json",
            },
        });

        const imageArrayBuffer = await image.arrayBuffer();
        const imageUint8Array = new Uint8Array(imageArrayBuffer);

        await fetch(url(QUERY_API_ASSET_BUILDER), {
            method: Method.POST,
            body: JSON.stringify({
                active: true,
                data: Array.from(imageUint8Array),
                name: image_url,
                file_hash: crypto.randomUUID(),
                mime_type: image.type,
            }),
            headers: {
                [AUTHORIZATION_REQUEST]: `Bearer ${token}`,
                [CONTENT_TYPE_REQUEST]: "application/json; charset=utf-8",
            },
        });

        return ok(JSON.stringify(response.json));
    } catch (e) {
        return handleRequestError(e as Error);
    }
}
