import { POST_DATABASE } from "@/config/shared/post.constants";
import { PAGE_ADMIN_LOGIN_PATH } from "@/config/shared/shared.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { getAssetPath } from "@/lib/server/get-asset-path";
import { render } from "@/lib/server/render";
import { Body, Head } from "@/pages/admin/layouts/template";
import svg from "@/pages/pages.svg";
import { HotReload } from "@/pages/hot-reload/hot-reload";

import { PostView, type PostViewProps } from "./post.view";

export async function handleRequest(req: Request): Promise<Response> {
    const url = new URL(req.url);

    try {
        const session = await getAdminUserSession(req);

        if (!session) {
            return Response.redirect(url.origin + PAGE_ADMIN_LOGIN_PATH);
        }

        const isExpired = await adminUserSession.isExpired(session);

        if (isExpired) {
            await adminUserSession.refresh(session);
        }
    } catch {
        return Response.redirect(url.origin + PAGE_ADMIN_LOGIN_PATH);
    }

    const db = new Database(POST_DATABASE);
    const result = db.query("SELECT * FROM post ORDER BY created_at DESC");

    const stylesPath = getAssetPath("dist/styles.css");
    const islandPath = getAssetPath("dist/admin/post/island/post.island.js");

    return new Response(
        render(
            <>
                <Head>
                    <title>Query Admin Post</title>
                    <link rel="stylesheet" href={stylesPath} />
                </Head>
                <Body class="overflow-y-scroll">
                    <PostView data={result as unknown as PostViewProps[]} />
                    { svg }
                    <script src={islandPath} type="module" />
                    <HotReload href={url.href} />
                </Body>
            </>,
        ),
        {
            headers: {
                "Content-Type": "text/html; charset=utf-8",
            },
        },
    );
}
