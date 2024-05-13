import type { ComponentChildren } from "preact";

type Tag = "button" | "a";
type Variant = "sm" | "md" | "lg" | "w-sm" | "w-md" | "w-lg" | "transparent" | "link";
type Color = "default" | "red";

export function Button({
    children = "Submit" as ComponentChildren,
    color = "default",
    tag = "button" as Tag,
    type = "button",
    variant = "sm" as Variant,
    ...props
}) {
    const colors: Record<Color, string> = {
        red: "bg-red-700 border-red-700 text-white hover:bg-red-500 hover:border-slate-500",
        default: "bg-slate-950 border border-slate-950 text-white hover:bg-slate-700 hover:border-slate-700",
    };
    const white_styles = "bg-white border border-slate-950 text-slate-950";
    const variants: Record<Variant, string> = {
        lg: `font-cal rounded h-12 px-8 text-base ${colors[color as Color]}`,
        md: `font-cal rounded h-10 px-6 text-sm ${colors[color as Color]}`,
        sm: `font-cal rounded h-8 px-4 text-xs ${colors[color as Color]}`,
        transparent: "",
        link: "bg-none border-none p-0 font-inherit cursor-pointer underline",
        "w-sm": `font-cal rounded h-8 px-4 text-xs ${white_styles} hover:bg-slate-100`,
        "w-md": `font-cal rounded h-10 px-6 text-sm ${white_styles} hover:bg-slate-100`,
        "w-lg": `font-cal rounded h-12 px-8 text-base ${white_styles} hover:bg-slate-100`,
    };

    return tag === "a" ? (
        <a {...props} className={`${variants[variant as Variant]} inline-flex items-center`}>
            {children}
        </a>
    ) : (
        <button {...props} className={`${variants[variant as Variant]}`} type={type}>
            {children}
        </button>
    );
}
