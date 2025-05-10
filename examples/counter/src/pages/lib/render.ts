import { htmlResponse } from "@/pages/lib/server/response";

// Legacy render function, now forwarding to htmlResponse
export const render = (html: ComponentChildren): string => {
    console.warn('render() is deprecated, use htmlResponse() instead');
    return `<!DOCTYPE html>${StringHTML(`<html lang="en">${html}</html>`)}`;
};