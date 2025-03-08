import { codeToHtml } from "./shiki.bundle";

async function highlightCode() {
    for (const el of Array.from(document.querySelectorAll("pre") || [])) {
        const code = el.querySelector("code");
        const lang =
            Array.from(code?.classList ?? [])
                .find((c) => c.startsWith("language-"))
                ?.replace("language-", "") || "js";

        if (el.textContent) {
            el.outerHTML = await codeToHtml(el.textContent, {
                lang,
                theme: "catppuccin-frappe",
            });
        }
    }
}

highlightCode();

// npx shiki-codegen \
//   --langs toml,sh,bash,js,jsx,tsx,javascript,typescript,html,docker,json,http,yaml,markdown \
//   --themes catppuccin-frappe \
//   --engine javascript \
//   ./src/pages/docs/island/shiki.bundle.ts
