// Define the interface for the Database class
export default class Database {
    /**
     * Creates a new Database instance.
     * @param {string} dbName - The name of the database to connect to.
     */
    constructor(dbName: string);

    /**
     * Executes a database query.
     * @template T - The expected return type of the query.
     * @param {string} query - The SQL query string to execute.
     * @param {Array<string | number | boolean | null>} [params] - Optional parameters for the query.
     * @returns {T} The result of the query.
     */
    query<T>(query: string, params?: Array<string | number | boolean | null>): T;
}

declare global {
    // biome-ignore lint/suspicious/noRedeclare: <explanation>
    var Database: Database;
}

declare module "*query:database" {
    export { Database };
}
