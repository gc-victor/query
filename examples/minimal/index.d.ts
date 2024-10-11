/// <reference lib="dom" />

declare global {
    const process: {
        env: {
            [key: string]: string | undefined;
        };
    };

    class Database {
        constructor(path: string);
        query<T>(sql: string, params?: unknown[]): T[];
    }

    declare module "*.html" {
        const content: string;
        export default content;
    }

    declare module "*query:database" {
        export class Database {
            constructor(path: string);
            query<T>(sql: string, params?: unknown[]): T[];
        }
    }
}

export type {};
