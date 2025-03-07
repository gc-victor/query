import { Database } from "query:database";
import { bad_request } from "./response";
import { query } from "./query";

const RATE_LIMIT_DATABASE = "rate_limit.sql";
const DEFAULT_MAX_REQUESTS = 10;
const DEFAULT_WINDOW_SECONDS = 30;

export interface RateLimitOptions {
    maxRequests?: number;
    windowSeconds?: number;
}

export async function checkRateLimit(req: Request, options: RateLimitOptions = {}): Promise<void> {
    const maxRequests = options.maxRequests || DEFAULT_MAX_REQUESTS;
    const windowSeconds = options.windowSeconds || DEFAULT_WINDOW_SECONDS;
    const ipAddress = getClientIp(req);
    const endpoint = new URL(req.url).pathname;

    try {
        await query(RATE_LIMIT_DATABASE, "DELETE FROM rate_limit WHERE window_start < strftime('%s', 'now', '-' || ? || ' seconds')", [
            windowSeconds,
        ]);

        const db = new Database(RATE_LIMIT_DATABASE);
        const result = db.query(
            `SELECT
                request_count as count
             FROM
                rate_limit
             WHERE
                ip_address = ?
             AND
                endpoint = ?
             AND
                window_start > strftime('%s', 'now', '-' || ? || ' seconds')
             ORDER BY
                window_start
             DESC
             LIMIT 1`,
            [ipAddress, endpoint, windowSeconds],
        ) as [{ count: number }];

        const currentCount = result[0]?.count ?? 0;

        if (currentCount >= maxRequests) {
            throw bad_request("Rate limit exceeded. Please try again later.");
        }

        await query(
            RATE_LIMIT_DATABASE,
            `INSERT INTO rate_limit (ip_address, endpoint, window_start, request_count)
             VALUES (
                ?,
                ?,
                strftime('%s', 'now'),
                1
             )
             ON CONFLICT(ip_address, endpoint)
             DO UPDATE SET
                request_count = request_count + 1,
                window_start = CASE
                    WHEN (strftime('%s', 'now') - window_start) >= ? THEN strftime('%s', 'now')
                    ELSE window_start
                END`,
            [ipAddress, endpoint, windowSeconds],
        );
    } catch (error) {
        if (error instanceof Response) {
            throw error;
        }
        throw bad_request("Error checking rate limit");
    }
}

function getClientIp(req: Request): string {
    const forwardedFor = req.headers.get("X-Forwarded-For");
    if (forwardedFor) {
        return forwardedFor.split(",")[0].trim();
    }
    return "127.0.0.1";
}
