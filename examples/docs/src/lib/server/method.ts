export const Method = {
    DELETE: "DELETE",
    GET: "GET",
    HEAD: "HEAD",
    OPTIONS: "OPTIONS",
    PATCH: "PATCH",
    POST: "POST",
    PUT: "PUT",
} as const;

export type Method = (typeof Method)[keyof typeof Method];
