import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { PAGE_ADMIN_LOGIN_PATH } from "@/config/shared/shared.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { getAssetPath } from "@/lib/server/get-asset-path";
import { render } from "@/lib/server/render";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Body, Head } from "@/pages/admin/layouts/template";
import svg from "@/pages/pages.svg";
import { {{ tablePascalCase }}View, type {{ tablePascalCase }}ViewProps } from "./{{ tableLowerCase }}.view";

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

    const db = new Database({{ tableConstantCase }}_DATABASE);
    const result = db.query("SELECT * FROM  {{ tableSnakeCase }} ORDER BY created_at DESC");

    const stylesPath = getAssetPath("dist/styles.css");
    const islandPath = getAssetPath("dist/admin/{{ tableLowerCase }}/island/{{ tableLowerCase }}.island.js");

    return new Response(
        render(
            <>
                <Head>
                    <title>Query Admin {{ tableCapitalCase }}</title>
                    <link rel="stylesheet" href={stylesPath} />
                </Head>
                <Body class="overflow-y-scroll">
                    <{{tablePascalCase }}View data={result as unknown as {{tablePascalCase }}ViewProps[]} />
                    { svg }
                    <script src={islandPath} type="module" />
                    <HotReload href={url.href} />
                </Body>
            </>
        ),
        {
            headers: {
                "Content-Type": "text/html; charset=utf-8",
            },
        },
    );
}
