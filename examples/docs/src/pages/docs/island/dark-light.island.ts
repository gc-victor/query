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

document.getElementById("themeToggle")?.addEventListener("click", toggleTheme);
