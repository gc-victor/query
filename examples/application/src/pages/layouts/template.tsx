import type { ComponentChildren } from "preact";

export function Head({ children }: { children?: ComponentChildren }) {
    return (
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta http-equiv="Content-Type" content="text/html" />

            {children}
        </head>
    );
}

export function Body({ children, ...props }: { children?: ComponentChildren; [key: string]: unknown | unknown[] }) {
    return <body {...props}>{children}</body>;
}
