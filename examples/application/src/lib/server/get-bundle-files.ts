import { bad_request } from "./responses";

export async function getNameHashed(fileName: string) {
    try {
        const db = new Database("query_asset.sql");
        const result = await db.query("SELECT name_hashed FROM asset WHERE name = ?", [fileName]);

        return result[0].name_hashed;
    } catch (e) {
        throw bad_request(`${(e as Error).toString()}. File: ${fileName}`);
    }
}
