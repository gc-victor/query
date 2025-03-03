export const QUERY_TOKEN = getQueryToken();

export class TokenError extends Error {}

function getQueryToken(): string {
    const db = new Database("query_config.sql");
    const result = db.query<{token: string}>("SELECT token FROM _config_token WHERE active = 1 AND name = ?", ["app"]);

    if (!Array.isArray(result) || result.length === 0) {
        throw new TokenError("Token not found.");
    }

    return result[0].token;
}
