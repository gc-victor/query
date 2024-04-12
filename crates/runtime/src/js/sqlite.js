class Database {
    #dbName;

    constructor(dbName) {
        this.#dbName = dbName;
    }

    query(query, params) {
        return JSON.parse(___sqlite_query(this.#dbName, query, JSON.stringify(params || [])));
    }
}

globalThis.Database = Database;
