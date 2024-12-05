import { IS_DEVELOPMENT } from "@/config/server/server.constants";
import { POST_DATABASE } from "@/config/shared/post.constants";
import { HOME_PATH } from "@/config/shared/shared.constants";
import { getAssetPath } from "@/lib/server/get-asset-path";
import { render } from "@/lib/server/render";
import { NOT_FOUND_CODE } from "@/lib/server/status";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Html404 } from "@/pages/layouts/404";
import { Layout } from "@/pages/layouts/layout";
import { Body, Head } from "@/pages/layouts/template";
import svg from "@/pages/pages.svg";
import { Post } from "./post";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const stylesPath = getAssetPath("dist/styles.css");

    try {
        const db = new Database(POST_DATABASE);
        const result = db.query("SELECT * FROM post WHERE slug = ?", [url.pathname.replace("/post/", "/")]) as {
            title: string,
            content: string,
            image_url: string,
            created_at: number,
        }[];
        const title = result[0].title;
        const content = result[0].content;
        const image_url = result[0].image_url;
        const created_at = new Date(result[0].created_at * 1000).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
        const datetime = new Date(result[0].created_at * 1000).toISOString();
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
                        <link rel="stylesheet" href={stylesPath} />
                    </Head>
                    <Body class="overflow-y-scroll">
                        <Layout>
                            <Post created_at={created_at} datetime={datetime} image_url={image_url} title={title} content={content} />
                        </Layout>
                        { svg }
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
                        <link rel="stylesheet" href={stylesPath} />
                    </Head>

                    <Body class="overflow-y-scroll">
                        <Layout>
                            <Html404 link={HOME_PATH} />
                        </Layout>
                        { svg }
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
