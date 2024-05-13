import { Button } from "@/pages/components/button";

interface ExcerptProps {
    image_url: string;
    title: string;
    datetime: string;
    created_at: string;
    excerpt: string;
    slug: string;
}

export function Excerpt({ image_url, title, datetime, created_at, excerpt, slug }: ExcerptProps) {
    return (
        <article className="space-y-4 mb-8 prose prose-slate mx-auto">
            <figure>
                <figcaption className="sr-only">Cover image</figcaption>
                <img
                    src={`/_/asset/${image_url}`}
                    alt="Cover of the article"
                    width="800"
                    height="100"
                    className="w-full h-[100px] object-cover object-center"
                />
            </figure>
            <div className="not-prose">
                <h2 className="text-2xl font-cal lg:text-3xl">{title}</h2>
                <p className="mt-4 text-slate-500">
                    Published on <time dateTime={datetime}>{created_at}</time>
                </p>
            </div>
            <p>{excerpt}</p>
            <p>
                <Button tag="a" href={slug}>
                    Read More
                </Button>
            </p>
        </article>
    );
}
