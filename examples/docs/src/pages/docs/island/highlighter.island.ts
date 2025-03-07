import { createHighlighter } from "shiki";

async function highlightCode() {
    const highlighter = await createHighlighter({
        langs: ["toml", "sh", "bash", "js", "javascript", "typescript", "html", "docker", "json", "http", "yaml"],
        themes: ["catppuccin-frappe"],
    });

    for (const el of Array.from(document.querySelectorAll("pre") || [])) {
        const code = el.querySelector("code");
        const lang =
            Array.from(code?.classList ?? [])
                .find((c) => c.startsWith("language-"))
                ?.replace("language-", "") || "js";

        if (el.textContent) {
            el.outerHTML = highlighter.codeToHtml(el.textContent, {
                lang,
                theme: "catppuccin-frappe",
            });
        }
    }
}

highlightCode();
