import { bad_request, internal_server_error } from "./responses";
import { url } from "./url";

interface FetcherResponse extends Response {
    [key: string]: unknown;
}

export async function fetcher(path: string, options: RequestInit): Promise<FetcherResponse> {
    const res = await fetch(url(path), options);

    if (res.status >= 500) {
        throw internal_server_error();
    }

    if (res.status >= 400) {
        const text = await res.text().catch(() => res.statusText);

        throw bad_request(text || res.statusText);
    }

    return { ...res, json: await res.json() };
}
