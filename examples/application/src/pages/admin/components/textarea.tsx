import type { ComponentChildren } from "preact";

import { FieldError } from "./filed-error";
import { Label } from "./label";

export function Textarea({
    children = "",
    id,
    label,
    ...props
}: { id: string; label: ComponentChildren; children?: ComponentChildren; [key: string]: unknown }) {
    return (
        <div class="relative space-y-1">
            <Label htmlFor={id} required={props["aria-required"] ? "true" : "false"}>
                {label}
            </Label>
            <textarea
                id={id}
                name={id}
                autocomplete="off"
                {...props}
                class="
                bg-transparent
                border
                border-slate-950
                h-64
                p-3
                placeholder-slate-600
                rounded
                text-slate-950
                w-full

                disabled:bg-slate-50
                disabled:text-slate-500
                disabled:border-slate-500

                peer

                aria-[invalid=true]:border-red-500
                aria-[invalid=true]:placeholder:text-red-500
                "
            >
                {children}
            </textarea>
            <FieldError id={id} />
        </div>
    );
}
