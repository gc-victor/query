export interface QueryTokenLoad {
    token: string;
}

export class QueryToken {
    #db;

    constructor() {
        this.#db = new Database("query_config.sql");
    }

    async load(name: string): Promise<QueryTokenLoad> {
        const result = await this.#db.query("SELECT token FROM _config_token WHERE active = 1 AND name = ?", [name]);

        if (result.length === 0) {
            throw new TokenError("Token not found.");
        }

        return result[0] as unknown as QueryTokenLoad;
    }
}

export const queryTokenService = new QueryToken();

export class TokenError extends Error {}
