import { TokenError } from "./error-classes";

export const QUERY_TOKEN = getQueryToken();

function getQueryToken(): string {
    try {
        const db = new Database("query_config.sql");
        const result = db.query<{token: string}>("SELECT token FROM _config_token WHERE active = 1 AND name = ?", ["app"]);

        if (!Array.isArray(result) || result.length === 0) {
            throw new TokenError("Token not found.");
        }

        return result[0].token;
    } catch (error) {
        throw new TokenError("Failed to retrieve token.");
    }
}
