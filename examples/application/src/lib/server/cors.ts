import { QUERY_APP_ALLOWED_ORIGIN } from "@/config/server/server.constants";
import { ACCESS_CONTROL_ALLOW_CREDENTIALS_RESPONSE, ACCESS_CONTROL_ALLOW_ORIGIN_RESPONSE } from "./header";

export function cors(res: Response) {
    res.headers.set(ACCESS_CONTROL_ALLOW_ORIGIN_RESPONSE, QUERY_APP_ALLOWED_ORIGIN || "*");
    res.headers.set(ACCESS_CONTROL_ALLOW_CREDENTIALS_RESPONSE, "true");
}
