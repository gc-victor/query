/// <reference lib="dom" />


declare global {
    // NOTE: To avoid editor ts error

    const process: {
        env: {
            [key: string]: string | undefined;
        };
    };

    class Database {
        constructor(path: string);
        query(sql: string, params?: unknown[] | Record<string, unknown>): Promise<Record<string, unknown>[]>;
    }
}

export type { };
