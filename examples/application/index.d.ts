/// <reference lib="dom" />

interface TableElementProps extends HTMLAttributes<HTMLTableElement> {
    url: string;
}

declare global {
    interface Window {
        SideDrawer: NodeElement;
    }

    const process: {
        env: {
            [key: string]: string | undefined;
        };
    };

    namespace JSX {
        interface Element {
            type: string;
            props: { [key: string]: unknown };
            children: unknown[];
        }

        interface IntrinsicElements {
            [elemName: string]: unknown;
            'table-element': TableElementProps;
        }
    }
    type ComponentChild = object | string | number | bigint | boolean | null | undefined;
    type ComponentChildren = ComponentChild[] | ComponentChild;
    const Fragment: (props: JSX.Fragment) => ComponentChildren;
    const StringHTML: (input: string) => string;

    class Database {
        constructor(path: string);
        query<T>(sql: string, params?: unknown[]): T[];
        query_cache<T>(query: string, params: Array<string | number | boolean | null>, ttl: number): T;
    }

    declare module "*query:database" {
        export class Database {
            constructor(path: string);
            query<T>(sql: string, params?: unknown[]): T[];
            query_cache<T>(query: string, params: Array<string | number | boolean | null>, ttl: number): T;
        }
    }

    declare module "*.svg" {
        const content: string;
        export default content;
    }
}

export type {};
