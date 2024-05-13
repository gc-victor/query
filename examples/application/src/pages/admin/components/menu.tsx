import { PAGE_ADMIN_POST_PATH } from "@/config/shared/post.constants";

export function Menu() {
    return (
        <menu className="flex items-center text-sm">
            <li className="mx-2">
                <a class="hover:underline" href={PAGE_ADMIN_POST_PATH}>
                    Post
                </a>
            </li>
        </menu>
    );
}
