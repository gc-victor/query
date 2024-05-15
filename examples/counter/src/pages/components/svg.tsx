import svg from "@/pages/pages.svg";

export function SVG() {
    // biome-ignore lint/security/noDangerouslySetInnerHtml: <explanation>
    return <div dangerouslySetInnerHTML={{ __html: svg }} />;
}
