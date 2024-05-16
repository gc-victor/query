import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { HOME_PATH } from "@/config/shared/shared.constants";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { getNameHashed } from "@/lib/server/get-bundle-files";
import { NOT_FOUND_CODE } from "@/lib/server/status";
import { render } from '@/lib/server/render';
import { Html404 } from "@/pages/layouts/404";
import { Layout } from "@/pages/layouts/layout";
import { Body, Head } from "@/pages/layouts/template";
import { SVG } from "@/pages/components/svg";
import { Button } from "@/pages/components/button";
import { {{ tablePascalCase }} } from "@/pages/{{ tableLowerCase }}/{{ tableLowerCase }}";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const stylesNameHashed = await getNameHashed("dist/styles.css");

    try {
        const db = new Database({{ tableConstantCase }}_DATABASE);
        const result = await db.query("SELECT * FROM  {{ tableSnakeCase }} WHERE uuid = ?", [url.pathname.replace("/{{ tableLowerCase }}/", "")]);
        const uuid = result[0]?.uuid as string;
        {% for column in columns %}
        const {{ column.columnNameCamelCase }} = result[0]?.{{ column.columnName }} as {{ column.columnTypeMatchTS }};
        {% endfor %}
        const createdAt = new Date((result[0].created_at as number) * 1000).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
        const datetime = new Date((result[0].created_at as number) * 1000).toISOString();

        return new Response(
            render(
                <>
                    <Head>
                        <title>Query {{ tableCapitalCase }} Item</title>
                        <link rel="stylesheet" href={`/_/asset/${stylesNameHashed}`} />
                    </Head>
                    <Body class="overflow-y-scroll">
                        <Layout>
                            <article>
                                <h1 class="font-cal text-4xl">{{ tableCapitalCase }} Item</h1>
                                <p class="text-slate-500">
                                    Published on <time datetime={datetime}>{createdAt}</time>
                                </p>
                                <{{ tablePascalCase }} {% for column in columns %} {{ column.columnNameCamelCase }}={{{ column.columnNameCamelCase }}}{% endfor %} createdAt={createdAt} datetime={datetime} />
                                <hr class="mt-8" />
                                <p class="mt-8">
                                    <Button tag="a" variant="md" href="/job-application/">
                                        {{ tableCapitalCase }} List
                                    </Button>
                                </p>
                            </article>
                        </Layout>
                        <SVG />
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
    } catch (error) {
        return new Response(
            render(
                <>
                    <Head>
                        <title>Page Not Found</title>
                        <link rel="stylesheet" href={`/_/asset/${stylesNameHashed}`} />
                    </Head>
                    <Body class="overflow-y-scroll">
                        <Layout>
                            <Html404 link={HOME_PATH} />
                        </Layout>
                        <HotReload href={url.href} />
                    </Body>
                </>
            ),
            {
                status: NOT_FOUND_CODE,
                headers: {
                    "Content-Type": "text/html; charset=utf-8",
                },
            },
        );
    }
}
