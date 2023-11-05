class Database {
    #dbName;

    constructor(dbName) {
        this.#dbName = dbName;
    }

    query(query, params) {
        return Deno.core.ops.op_sqlite_query_extension(this.#dbName, query, params);
    }

    execute(query, params) {
        return Deno.core.ops.op_sqlite_execute_extension(this.#dbName, query, params);
    }
}

globalThis.Database = Database;
