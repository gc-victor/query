const queryFunction = `
import { Database } from 'query:database';

globalThis.___handleRequest = async () => {
    try {
        const db = new Database("hurl-query.sql");
        console.log("Database", Database.toString());
        
        const create = db.query("CREATE TABLE IF NOT EXISTS test (value TEXT)");
        const insert = db.query("INSERT INTO test (value) VALUES (?)", ["Hello, World!"]);
        const insert1 = db.query("INSERT INTO test (value) VALUES (?)", ["Bye 1!"]);
        const insert2 = db.query("INSERT INTO test (value) VALUES (?)", ["Bye 2!"]);
        const inserts = db.query("SELECT * FROM test");
        const update = db.query("UPDATE test SET value = ? WHERE rowid = ?", ["Bye!", insert.rowid]);
        const deleteRows = db.query("DELETE FROM test WHERE rowid != ?", [insert.rowid]);
        const rows = db.query("SELECT * FROM test");
        
        const response = JSON.stringify({ create, insert, insert1, insert2, inserts, update, deleteRows, rows });

        return new Response(response, { status: 200 });
    } catch (e) {
        console.error(e.message + "\\n" + (e.stack || ""));
        return new Response(e.message + "\\n" + (e.stack || ""), { status: 500 });
    }
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(queryFunction)).toString()}]`);
