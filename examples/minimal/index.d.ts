/// <reference lib="dom" />

declare global {
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
        }
    }
    type ComponentChild = object | string | number | bigint | boolean | null | undefined;
    type ComponentChildren = ComponentChild[] | ComponentChild;
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
        }
    }

    declare module "*.html" {
        const content: string;
        export default content;
    }
}

export type {};
