import type { ComponentChildren } from "preact";

export function Legend({ children = "", ...props }: { children?: ComponentChildren; [key: string]: unknown }) {
    return (
        <legend class="font-cal text-2xl text-slate-950" {...props}>
            {children}
        </legend>
    );
}
