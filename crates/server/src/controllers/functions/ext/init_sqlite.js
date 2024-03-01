class Database {
    #dbName;

    constructor(dbName) {
        this.#dbName = dbName;
    }

    query(query, params) {
        return JSON.parse(Deno.core.ops.op_sqlite_query_extension(this.#dbName, query, JSON.stringify(params || [])));
    }

    execute(query, params) {
        return JSON.parse(Deno.core.ops.op_sqlite_execute_extension(this.#dbName, query, JSON.stringify(params || [])));
    }
}

globalThis.Database = Database;
