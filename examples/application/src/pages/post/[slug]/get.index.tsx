import { IS_DEVELOPMENT } from "@/config/server/server.constants";
import { POST_DATABASE } from "@/config/shared/post.constants";
import { HOME_PATH } from "@/config/shared/shared.constants";
import { getNameHashed } from "@/lib/server/get-bundle-files";
import { render } from "@/lib/server/render";
import { NOT_FOUND_CODE } from "@/lib/server/status";
import { SVG } from "@/pages/components/svg";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Html404 } from "@/pages/layouts/404";
import { Layout } from "@/pages/layouts/layout";
import { Body, Head } from "@/pages/layouts/template";
import { Post } from "./post";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const stylesNameHashed = await getNameHashed("dist/styles.css");

    try {
        const db = new Database(POST_DATABASE);
        const result = await db.query("SELECT * FROM post WHERE slug = ?", [url.pathname.replace("/post/", "/")]);
        const title = (result[0].title) as string;
        const content = result[0].content as string;
        const image_url = result[0].image_url as string;
        const created_at = new Date((result[0].created_at as number) * 1000).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
        const datetime = new Date((result[0].created_at as number) * 1000).toISOString();
        const headers = {
            "Content-Type": "text/html; charset=utf-8",
            "Query-Cache-Control": "max-age=360000",
        };

        if (IS_DEVELOPMENT) {
            headers["Query-Cache-Control"] = "max-age=0";
        }

        return new Response(
            render(
                <>
                    <Head>
                        <title>{title}</title>
                        <link rel="stylesheet" href={`/_/asset/${stylesNameHashed}`} />
                    </Head>
                    <Body class="overflow-y-scroll">
                        <Layout>
                            <Post created_at={created_at} datetime={datetime} image_url={image_url} title={title} content={content} />
                        </Layout>
                        <SVG />
                        <HotReload href={url.href} />
                    </Body>
                </>,
            ),
            {
                headers,
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
                        <SVG />
                        <HotReload href={url.href} />
                    </Body>
                </>,
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
