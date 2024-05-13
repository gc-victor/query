import type { ComponentChildren } from "preact";

export function Label({
    children,
    htmlFor,
    required,
    ...props
}: { htmlFor: string; children?: ComponentChildren; [key: string]: unknown | unknown[] }) {
    return (
        <label
            for={htmlFor}
            class="
                font-cal
                leading-none
                text-lg
            "
            {...props}
        >
            {children}
            {required === "true" ? (
                <span class="text-red-500" aria-hidden="true">
                    *
                </span>
            ) : (
                ""
            )}
        </label>
    );
}
