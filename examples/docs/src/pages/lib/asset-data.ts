import { Database } from "query:database";
import { NotFoundError } from "./types";

export function getAssetData<T>(assetPath: string): T {
    const db = new Database("query_asset.sql");
    const [result] = db.query("SELECT data FROM asset WHERE name = ? LIMIT 1", [assetPath]) as [{ data: Uint8Array }];

    if (!result) {
        throw new NotFoundError(`Asset not found: ${assetPath}`);
    }

    try {
        const jsonString = new TextDecoder().decode(result.data);
        return JSON.parse(jsonString) as T;
    } catch (e: unknown) {
        const error = e as Error;
        
        throw new Error(`Failed to parse JSON data: ${error.message}\\n${error.stack || ""}`);
    }
}
