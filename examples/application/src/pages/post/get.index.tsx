import { Database } from "query:database";

import { PAGE_POST_PATH, POST_DATABASE } from "@/config/shared/post.constants";
import { getAssetPath } from "@/lib/server/get-asset-path";
import { render } from "@/lib/server/render";
import { HotReload } from "@/pages/hot-reload/hot-reload";
import { Layout } from "@/pages/layouts/layout";
import { Body, Head } from "@/pages/layouts/template";
import svg from "@/pages/pages.svg";
import { Excerpt } from "./excerpt";

export async function handleRequest(req: Request) {
    const url = new URL(req.url);

    const db = new Database(POST_DATABASE);
    const result = db.query("SELECT title, content, slug, image_url, created_at FROM post ORDER BY created_at DESC") as {
        title: string
        content: string
        slug: string
        image_url: string
        created_at: number
    }[];

    const posts = result.map((post) => {
        const title = post.title;
        const content = post.content;
        const slug = post.slug;
        const image_url = result[0].image_url;
        const excerpt = content?.match(/<p>([\s\S]+?)<\/p>/)?.[1];

        const created_at = new Date((post.created_at as number) * 1000).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
        const datetime = new Date((post.created_at as number) * 1000).toISOString();

        return {
            image_url: image_url as string,
            title: title as string,
            datetime: datetime  as string,
            created_at: created_at as string,
            excerpt: excerpt as string,
            slug: `${PAGE_POST_PATH}${slug}` as string,
        };
    });

    const stylesPath = getAssetPath("dist/styles.css");

    return new Response(
        render(
            <>
                <Head>
                    <title>Query Blog</title>
                    <link rel="stylesheet" href={stylesPath} />
                </Head>
                <Body class="overflow-y-scroll">
                    <Layout>
                        <div class="flex flex-col space-y-8">
                            {posts.map((post) => {
                                // biome-ignore lint/correctness/useJsxKeyInIterable: <explanation>
                                return <Excerpt {...post} />
                            })}
                        </div>
                    </Layout>
                    { svg }
                    {/* <HotReload href={url.href} /> */}
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
