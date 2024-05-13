import { SET_COOKIE_RESPONSE } from "@/lib/server/header";

export type Session = string;
export type Token = string;

export interface AdminUserSessionLoad {
    token: Token;
    expires_at: number;
}

export class AdminUserSession {
    #db;

    constructor() {
        this.#db = new Database("admin_user_session.sql");
    }

    async save(session: Session, outerToken: Token): Promise<void> {
        // set the expiry time as 120s after the current time
        const now = new Date();
        const expiresAt = +new Date(+now + 120 * 1000);

        await this.#db.query("INSERT INTO session (session, token, expires_at) VALUES (?, ?, ?)", [session, outerToken, expiresAt]);

        this.clearExpired();
    }

    async load(session: Session): Promise<AdminUserSessionLoad> {
        const result = await this.#db.query("SELECT token, expires_at FROM session WHERE session = ?", [session]);

        return result[0] as unknown as AdminUserSessionLoad;
    }

    async clear(session: Session): Promise<void> {
        await this.#db.query("DELETE FROM session WHERE session = ?", [session]);
    }

    async isExpired(session: Session): Promise<boolean> {
        const now = new Date();
        const value = await this.load(session);

        if (value) {
            return value.expires_at < now.getTime();
        }

        return true;
    }

    async clearExpired(): Promise<void> {
        const now = new Date().getTime();

        await this.#db.query("DELETE FROM session WHERE expires_at < ?", [now]);
    }

    async refresh(session: Session): Promise<void> {
        const value = await this.load(session);

        if (value) {
            const now = new Date();
            const expiresAt = new Date(+now + 60 * 1000).getTime();

            await this.#db.query("UPDATE session SET expires_at = ? WHERE session = ?", [expiresAt, session]);
        }
    }
}

export class AdminUserSessionError extends Error {}

export const adminUserSession = new AdminUserSession();

export async function getAdminUserSession(req: Request): Promise<string> {
    const cookie = req.headers.get("cookie");

    if (!cookie) throw new AdminUserSessionError("There isn't a Set Cookie header.");

    const match = cookie.match(/session=([\w-]+)/);

    if (!match) throw new AdminUserSessionError("Session cookie not found.");

    const session = match[1];
    const hasSession = !(await adminUserSession.load(session));

    if (hasSession) {
        throw new AdminUserSessionError("Session not found.");
    }

    return decodeURIComponent(session);
}

export async function setAdminUserSession(token: string, res: Response) {
    const session = crypto.randomUUID();

    await adminUserSession.save(session, token);

    res.headers.set(
        SET_COOKIE_RESPONSE,
        `session=${session}; Path=/; Expires=3600000; Max-Age=3600000; HttpOnly; SameSite=Strict; Secure;`,
    );
}
