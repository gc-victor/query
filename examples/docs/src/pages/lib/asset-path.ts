import { Database } from "query:database";
import { bad_request } from "@/lib/server/response";

export function assetPath(fileName: string) {
    try {
        const db = new Database("query_asset.sql");
        const result = db.query_cache("SELECT name_hashed FROM asset WHERE name = ?", [fileName], 86400000) as {
            name_hashed: string;
        }[];

        return `/_/asset/${result[0]?.name_hashed || fileName}`;
    } catch (e) {
        throw bad_request(`${(e as Error).toString()}. File: ${fileName}`);
    }
}
