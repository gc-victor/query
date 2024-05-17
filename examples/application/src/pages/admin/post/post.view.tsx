import { PAGE_ADMIN_POST_PATH } from "@/config/shared/post.constants";
import { htmlspecialchars } from "@/lib/server/htmlspecialchars";
import { Button } from "@/pages/admin/components/button";
import { Header } from "@/pages/admin/components/header";
import { ID_DRAWER_COMPONENT, ID_NEW_ITEM } from "@/pages/admin/utils/constants";
import { PostFormView } from "./post.form.view";

export interface PostViewProps {
    id: number;
    uuid: string;
    title: string;
    slug: string;
    content: string;
    image_url: string;
    created_at: number;
    updated_at: number;
}

export function PostView({ data }: { data: PostViewProps[] }) {
    return (
        <>
            <Header>
                {/* NOTE: The click handler is managed by table-element */}
                <Button id={ID_NEW_ITEM}>New Post</Button>
            </Header>
            <main>
                <drawer-element id={ID_DRAWER_COMPONENT} class="hidden" right>
                    <PostFormView />
                </drawer-element>
                <div data-hot-reload-scroll="table-wrapper" class="h-[calc(100lvh_-_65px)] relative overflow-x-auto overflow-y-auto">
                    <div class="absolute bg-slate-100 h-10 w-full -z-10" />
                    <table is="table-element" data-url={PAGE_ADMIN_POST_PATH} class="text-left rtl:text-right w-max">
                        <thead class="font-cal h-10 text-xs uppercase">
                            <tr>
                                <th scope="col" class="bg-slate-100 px-4 py-3 text-center w-id z-10">
                                    <span className="sr-only">Visit</span>
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 text-center w-id z-10">
                                    id
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 w-id z-10">
                                    uuid
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 w-string z-10">
                                    title
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 w-string z-10">
                                    slug
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 w-string z-10">
                                    image_url
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 w-text z-10">
                                    content
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 text-center w-timestamp z-10">
                                    created_at
                                </th>
                                <th scope="col" class="bg-slate-100 px-4 py-3 text-center w-timestamp z-10">
                                    updated_at
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {data.length === 0 ? (
                                <tr>
                                    <td>{}</td>
                                    <td>{}</td>
                                    <td>{}</td>
                                    <td class="py-4 text-center" colspan={4}>
                                        No Post
                                    </td>
                                    <td>{}</td>
                                    <td>{}</td>
                                </tr>
                            ) : (
                                data.map((post) => (
                                    <tr
                                        key={post.uuid}
                                        data-uuid={post.uuid}
                                        class="border-b border-slate-100 h-8 relative text-sm hover:bg-slate-50"
                                    >
                                        <td class="px-4 text-center">
                                            <span className="relative z-10">
                                                <Button tag="a" href={`/post${post.slug}`}>
                                                    Visit
                                                </Button>
                                            </span>
                                        </td>
                                        <td class="px-4 text-center">
                                            {/*
                                                    @see:
                                                    https://adrianroselli.com/2020/02/block-links-cards-clickable-regions-etc.html#Update02
                                                    https://adrianroselli.com/2020/02/block-links-cards-clickable-regions-etc.html#comment-246683
                                                */}
                                            <button
                                                class="
                                                    after:content-['']
                                                    after:block
                                                    after:absolute
                                                    after:inset-0

                                                    focus:after:outline
                                                    focus:after:outline-2
                                                    focus:after:outline-slate-950
                                                "
                                                type="button"
                                            >
                                                <span class="sr-only">Edit</span>
                                            </button>
                                            {String(post.id)}
                                        </td>
                                        <td class="px-4">
                                            <div class="w-uuid truncate">{post.uuid}</div>
                                        </td>
                                        <td class="px-4">
                                            <div class="w-string truncate">{post.title}</div>
                                        </td>
                                        <td class="px-4">
                                            <div class="w-string truncate">{post.slug}</div>
                                        </td>
                                        <td class="px-4">
                                            <div class="w-text truncate">{post.image_url}</div>
                                        </td>
                                        <td class="px-4">
                                            <div class="max-h-8 w-text truncate">{htmlspecialchars(post.content)}</div>
                                        </td>
                                        <td class="px-4 text-center">
                                            <div class="m-auto w-timestamp">
                                                <div>{date(post.created_at)}</div>
                                                <div class="text-slate-500 text-xs">{time(post.created_at)}</div>
                                            </div>
                                        </td>
                                        <td class="px-4 text-center">
                                            <div class="m-auto w-timestamp">
                                                <div>{date(post.updated_at)}</div>
                                                <div class="text-slate-500 text-xs">{time(post.updated_at)}</div>
                                            </div>
                                        </td>
                                    </tr>
                                ))
                            )}
                        </tbody>
                    </table>
                </div>
            </main>
        </>
    );
}

function date(timestamp: number) {
    return new Date(timestamp * 1000).toLocaleDateString();
}

function time(timestamp: number) {
    const date = new Date(timestamp * 1000);
    const hours = String(date.getUTCHours()).padStart(2, "0");
    const minutes = String(date.getUTCMinutes()).padStart(2, "0");
    const seconds = String(date.getUTCSeconds()).padStart(2, "0");

    return `${hours}:${minutes}:${seconds} UTC`;
}
