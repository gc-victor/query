import { PAGE_ADMIN_POST_PATH } from "@/config/shared/post.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { getAssetPath } from "@/lib/server/get-asset-path";
import { render } from "@/lib/server/render";
import svg from "@/pages/pages.svg";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Body, Head } from "@/pages/layouts/template";
import { LoginView } from "./login.view";

export async function handleRequest(req: Request): Promise<Response> {
    const url = new URL(req.url);

    try {
        const session = await getAdminUserSession(req);

        if (session) {
            adminUserSession.refresh(session);

            return Response.redirect(url.origin + PAGE_ADMIN_POST_PATH);
        }
    } catch {}

    const stylesPath = getAssetPath("dist/styles.css");
    const islandPath = getAssetPath("dist/admin/login/login.island.js");

    return new Response(
        render(
            <>
                <Head>
                    <title>Query Admin Login</title>
                    <link rel="stylesheet" href={stylesPath} />
                </Head>
                <Body class="bg-slate-950 text-white bg-gradient-to-b from-slate-900 to-slate-950 overflow-y-scroll">
                    <LoginView />
                    { svg }
                    <script src={islandPath} type="module" />
                    <HotReload href={url.href} />
                </Body>
            </>,
        ),
        {
            headers: {
                "Content-Type": "text/html;charset=utf-8",
            },
        },
    );
}
