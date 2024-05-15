export async function getNameHashed(fileName: string) {
    try {
        const db = new Database("query_asset.sql");

        const result = await db.query("SELECT name_hashed FROM asset WHERE name = ?", [fileName]);

        return `/_/asset/${result[0].name_hashed}`;
    } catch (e: unknown) {
        console.error("getNameHashed", e);
    }
}
