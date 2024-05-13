import type { ComponentChildren } from "preact";

import { API_ADMIN_LOGOUT_PATH } from "@/config/shared/shared.constants";
import { Button } from "./button";
import { Menu } from "./menu";

export function Header({ children }: { children?: ComponentChildren }) {
    return (
        <header class="flex items-center justify-between px-4">
            <figure class="flex items-center h-16">
                <svg height="24" width="64">
                    <title>Query Logo</title>
                    <use href="#query-logo" fill="rgb(2 6 23)" height="24" width="64" />
                </svg>
            </figure>
            <nav class="flex">
                <Menu />
                <span class="ml-4">{children}</span>
                <span class="ml-4">
                    <Button tag="a" variant="w-sm" href={API_ADMIN_LOGOUT_PATH}>
                        Logout
                    </Button>
                </span>
            </nav>
        </header>
    );
}
