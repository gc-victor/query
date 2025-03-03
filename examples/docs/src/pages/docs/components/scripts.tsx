export const Scripts = () => (
    <>
        <script>
            ${`
                const htmlElement = document.documentElement;
                const storedTheme = localStorage.getItem("theme");
                const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
                const initialTheme = storedTheme || systemTheme;

                if (initialTheme === "dark") {
                    htmlElement.classList.add("dark");
                }

                const toggleTheme = () => {
                    const isDark = htmlElement.classList.contains("dark");
                    if (isDark) {
                        htmlElement.classList.remove("dark");
                        localStorage.theme = "light";
                    } else {
                        htmlElement.classList.add("dark");
                        localStorage.theme = "dark";
                    }
                };

                document.getElementById("themeToggle").addEventListener("click", toggleTheme);
            `}
        </script>
        <script type="module">
            ${`
                import { codeToHtml } from "https://esm.sh/shiki@1.0.0";
                const pre = document.querySelectorAll("pre");

                pre.forEach(async (el) => {
                    const code = el.querySelector("code");
                    const lang = Array.from(code.classList).find(c => c.startsWith('language-'))?.replace('language-', '') || 'js';

                    console.log(lang);

                    el.outerHTML = await codeToHtml(el.textContent, {
                        lang,
                        theme: "catppuccin-frappe",
                    });
                });
            `}
        </script>
    </>
);
