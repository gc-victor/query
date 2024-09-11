const databaseFunction = `
import { Database as Db } from 'query:database';
import Database from 'query:database';

globalThis.___handleRequest = async () => {
    try {
        const db1 = new Database("hurl_database_1.db");
        console.log("Database", Database.toString());
        
        const db2 = new Db("hurl_database_2.db");
        console.log("Db", Db.toString());

        return new Response("OK", { status: 200 });
    } catch (e) {
        console.error(e.message + "\\n" + (e.stack || ""));
        return new Response(e.message + "\\n" + (e.stack || ""), { status: 500 });
    }
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(databaseFunction)).toString()}]`);
