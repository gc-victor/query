export interface PostType {
    image_url: string;
    datetime: string;
    created_at: string;
    title: string;
    content: string;
}

export function Post({ image_url, datetime, created_at, title, content }: PostType) {
    // console.error(JSON.stringify({ content }));

    return (
        <article>
            <header class="text-center">
                <figure>
                    <figcaption class="sr-only">Cover image</figcaption>
                    <img
                        src={`/_/asset/${image_url}`}
                        alt="Cover of the post"
                        width="800"
                        height="600"
                        class="w-full h-64 object-cover object-center"
                    />
                </figure>
                <p class="mt-8 text-sm text-slate-500">
                    Published on <time datetime={datetime}>{created_at}</time>
                </p>
                <h1 class="font-cal font-bold mt-4 text-5xl text-slate-900">{title}</h1>
            </header>
            <div
                id="post"
                class="mt-8 text-slate-700 space-y-4 leading-6"
                // biome-ignore lint/security/noDangerouslySetInnerHtml: <explanation>
                dangerouslySetInnerHTML={{
                    __html: content,
                }}
            />
        </article>
    );
}
