export function slugify(str: string): string {
    return encodeURIComponent(String(str).trim().toLowerCase().replace(/\s+/g, "-"));
}
