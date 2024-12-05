export function getAssetPath(fileName: string) {
    try {
        const db = new Database("query_asset.sql");
        const result = db.query("SELECT name_hashed FROM asset WHERE name = ?", [fileName]) as { name_hashed: string }[];
    
        return `/_/asset/${result[0].name_hashed}`;
    } catch (e: unknown) {
        console.error("getPath", e);
    }
}
