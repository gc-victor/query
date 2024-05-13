import { PAGE_ADMIN_POST_PATH } from "@/config/shared/post.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { getNameHashed } from "@/lib/server/get-bundle-files";
import { render } from "@/lib/server/render";
import { SVG } from "@/pages/components/svg";
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

    const stylesNameHashed = await getNameHashed("dist/styles.css");
    const islandNameHashed = await getNameHashed("dist/admin/login/login.island.js");

    return new Response(
        render(
            <>
                <Head>
                    <title>Query Admin Login</title>
                    <link rel="stylesheet" href={`/_/asset/${stylesNameHashed}`} />
                </Head>
                <Body class="bg-slate-950 text-white bg-gradient-to-b from-slate-900 to-slate-950 overflow-y-scroll">
                    <LoginView />
                    <SVG />
                    <script src={`/_/asset/${islandNameHashed}`} type="module" />
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
