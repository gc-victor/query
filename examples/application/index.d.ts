/// <reference lib="dom" />

import type { JSX as PreactJSX } from 'preact';

declare module 'preact' {
    namespace JSX {
        interface IntrinsicElements extends PreactJSX.IntrinsicElements {
            'table-element': TableElementProps;
        }
    }
}

interface TableElementProps extends HTMLAttributes<HTMLTableElement> {
    url: string;
}

interface Table extends React.HTMLAttributes {
    url: string;
}

declare global {
    // NOTE: To avoid editor ts error

    interface Window {
        SideDrawer: NodeElement;
    }

    const process: {
        env: {
            [key: string]: string | undefined;
        };
    };

    class Database {
        constructor(path: string);
        query(sql: string, params?: unknown[] | Record<string, unknown>): Promise<Record<string, unknown>[]>;
    }

    declare module "*.html" {
        const content: string;
        export default content;
    }

    declare module "*.svg" {
        const content: string;
        export default content;
    }
}
