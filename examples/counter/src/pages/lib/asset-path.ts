import { Database } from "query:database";

export function assetPath(fileName: string) {
    try {
        const db = new Database("query_asset.sql");
        const result = db.query("SELECT name_hashed FROM asset WHERE name = ?", [fileName]) as { name_hashed: string }[];
        return `/_/asset/${result[0].name_hashed}`;
    } catch (e) {
        console.error("assetPath", e);
        return "";
    }
}