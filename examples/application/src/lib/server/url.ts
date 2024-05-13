import { QUERY_APP_QUERY_SERVER } from "@/config/server/server.constants";

export function url(path: string): string {
    return `${QUERY_APP_QUERY_SERVER}${path}`;
}
