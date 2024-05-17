import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { getNameHashed } from "@/lib/server/get-bundle-files";
import { render } from '@/lib/server/render';
import { Layout } from "@/pages/layouts/layout";
import { Body, Head } from "@/pages/layouts/template";
import { SVG } from "@/pages/components/svg";
import { {{ tablePascalCase }} } from "@/pages/{{ tableLowerCase }}/{{ tableLowerCase }}";

export interface {{ tablePascalCase }}Type {
    uuid: string;
    {% for column in columns %}
    {{ column.columnNameCamelCase }}: {{ column.columnTypeMatchTS }};
    {% endfor %}
    datetime: string;
    createdAt: string;
}

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const db = new Database({{ tableConstantCase }}_DATABASE);
    const result = await db.query("SELECT * FROM  {{ tableSnakeCase }} ORDER BY created_at DESC");
    const {{ tableCamelCase }}List = result.map(({{ tableCamelCase }}): {{ tablePascalCase }}Type => {
        {% for column in columns %}
        const {{ column.columnNameCamelCase }} = {{ tableCamelCase }}.{{ column.columnName }} as {{ column.columnTypeMatchTS }};
        {% endfor %}

        const created_at = new Date(({{ tableCamelCase }}.created_at as number) * 1000).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
        const datetime = new Date(({{ tableCamelCase }}.created_at as number) * 1000).toISOString();

        return {
            uuid: {{ tableCamelCase }}.uuid as string,
            {% for column in columns %}
            {{ column.columnNameCamelCase }},
            {% endfor %}
            createdAt: created_at as string,
            datetime: datetime as string,
        };
    });

    const stylesNameHashed = await getNameHashed("dist/styles.css");

    return new Response((
            render(
                <>
                    <Head>
                        <title>Query {{ tableCapitalCase }} List</title>
                        <link rel="stylesheet" href={`/_/asset/${stylesNameHashed}`} />
                    </Head>
                    <Body class="overflow-y-scroll">
                        <Layout>
                            <div class="flex flex-col space-y-8">
                                <h1 class="font-cal text-4xl">{{ tableCapitalCase }} List</h1>
                                { {{ tableCamelCase }}List.map(({{ tableCamelCase }}: {{ tablePascalCase }}Type) => (
                                    <article key={ {{ tableCamelCase }}.uuid } class="rounded-lg border p-8 shadow-sm">
                                        <h2 class="font-cal text-2xl">
                                            <a class="underline" href={`/{{ tableLowerCase }}/${ {{ tableCamelCase }}.uuid }`}>
                                                {{ tableCapitalCase }} Item
                                            </a>
                                        </h2>
                                        <p class="text-sm text-slate-500">
                                            Published on <time datetime={ {{ tableCamelCase }}.datetime }>{ {{ tableCamelCase }}.createdAt }</time>
                                        </p>
                                        <{{ tablePascalCase }} {...{{ tableCamelCase }}} />
                                    </article>
                                )) }
                            </div>
                        </Layout>
                        <SVG />
                        <HotReload href={url.href} />
                    </Body>
                </>
            )
        ), {
        headers: {
            "Content-Type": "text/html; charset=utf-8",
        },
    });
}
