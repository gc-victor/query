import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { PAGE_ADMIN_LOGIN_PATH } from "@/config/shared/shared.constants";
import { adminUserSession, getAdminUserSession } from "@/lib/server/admin-user-session";
import { getNameHashed } from "@/lib/server/get-bundle-files";
import { render } from "@/lib/server/render";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Body, Head } from "@/pages/admin/layouts/template";
import { SVG } from "@/pages/components/svg";
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
    const result = await db.query("SELECT * FROM  {{ tableSnakeCase }} ORDER BY created_at DESC");

    const stylesNameHashed = await getNameHashed("dist/styles.css");
    const islandNameHashed = await getNameHashed("dist/admin/{{ tableLowerCase }}/island/{{ tableLowerCase }}.island.js");

    return new Response(
        render(
            <>
                <Head>
                    <title>Query Admin {{ tableCapitalCase }}</title>
                    <link rel="stylesheet" href={`/_/asset/${stylesNameHashed}`} />
                </Head>
                <Body class="overflow-y-scroll">
                    <{{tablePascalCase }}View data={result as unknown as {{tablePascalCase }}ViewProps[]} />
                    <SVG />
                    <script src={`/_/asset/${islandNameHashed}`} type="module" />
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
