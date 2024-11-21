export class Database {
    #dbName;

    constructor(dbName) {
        this.#dbName = dbName;
    }

    query(query, params) {
        return JSON.parse(___sqlite_query(this.#dbName, query, JSON.stringify(params || []), 0));
    }

    query_cache(query, params, ttl) {
        return JSON.parse(___sqlite_query(this.#dbName, query, JSON.stringify(params || []), ttl));
    }
}

globalThis.Database = Database;

export default Database;
