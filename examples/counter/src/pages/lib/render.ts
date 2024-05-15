import type { ComponentChildren, VNode } from "preact";
import { render as r } from "preact-render-to-string";

export const render = (html: VNode<ComponentChildren>): string => {
    return `<!DOCTYPE html><html lang="en">${r(html)}</html>`;
};
