import type { ComponentChildren } from "preact";

import { PAGE_POST_PATH } from "@/config/shared/post.constants";

export function Layout({ children }: { children?: ComponentChildren }) {
    return (
        <div class="flex flex-col min-h-screen bg-white">
            <header class="w-full py-2 px-8 bg-white shadow">
                <div class="container mx-auto flex items-center h-16 justify-between ">
                    <figure>
                        <svg height="32" width="84">
                            <title>Query Logo</title>
                            <use href="#query-logo" fill="rgb(2 6 23)" height="32" width="84" />
                        </svg>
                    </figure>
                    <nav class="flex items-center">
                        <menu class="flex space-x-4">
                            <li>
                                <a class="text-slate-800 hover:underline" href={PAGE_POST_PATH}>
                                    Post
                                </a>
                            </li>
                        </menu>
                    </nav>
                </div>
            </header>
            <main class="flex-1 space-y-8 w-full max-w-4xl px-4 py-8 mx-auto md:px-8 md:py-16">{children}</main>
            <footer class="w-full h-12 flex justify-center items-center border-t border-slate-200 bg-white">
                <p class="text-sm text-slate-600">© 2024 Query App. All rights reserved.</p>
            </footer>
        </div>
    );
}
