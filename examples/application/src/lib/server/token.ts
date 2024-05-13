import { AUTHORIZATION_REQUEST } from "@/lib/server/header";

export type QueryToken = string;
export type PublicToken = string;

export interface TokenLoad {
    active: boolean;
    name: string;
    query_token: QueryToken;
    public_token: PublicToken;
}

export class Token {
    #db;

    constructor() {
        this.#db = new Database("token.sql");
    }

    async save(name: string, queryToken: QueryToken): Promise<void> {
        await this.#db.query("INSERT INTO token (name, query_token) VALUES (?, ?, ?)", [name, queryToken]);
    }

    async load(name: string, req: Request): Promise<TokenLoad> {
        const authorizationRequest = req.headers.get(AUTHORIZATION_REQUEST);

        if (!authorizationRequest) {
            throw new TokenError("Authorization header is missing.");
        }

        const publicToken = authorizationRequest.replace("Bearer ", "");

        const result = await this.#db.query("SELECT * FROM token WHERE active = 1 AND name = ? AND public_token = ?", [name, publicToken]);

        if (result.length === 0) {
            throw new TokenError("Token not found.");
        }

        return result[0] as unknown as TokenLoad;
    }

    async clear(name: string): Promise<void> {
        await this.#db.query("DELETE FROM token WHERE name = ?", [name]);
    }

    async activate(name: string): Promise<void> {
        await this.#db.query("UPDATE token SET active = 1 WHERE name = ?", [name]);
    }

    async deactivate(name: string): Promise<void> {
        await this.#db.query("UPDATE token SET active = 0 WHERE name = ?", [name]);
    }

    async refreshToken(name: string): Promise<void> {
        await this.#db.query("UPDATE session SET public_token = uuid() WHERE name = ?", [name]);
    }
}

export class TokenError extends Error {}

export const tokenService = new Token();

export async function validateToken(name: string, req: Request): Promise<void> {
    try {
        await tokenService.load(name, req);
    } catch (e) {
        throw new TokenError((e as Error).message);
    }
}
