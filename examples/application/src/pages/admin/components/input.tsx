import { FieldError } from "./filed-error";
import { Label } from "./label";

export function Input({ label, id, ...props }: { label: string; id: string; [key: string]: unknown }) {
    const fileStyles = `
    file:-mx-3
    file:-my-3
    file:[border-inline-end-width:1px]
    file:[margin-inline-end:0.75rem]
    file:bg-slate-950
    file:invalid:bg-red-500
    file:border-0
    file:border-inherit
    file:border-solid
    file:px-3
    file:py-3
    file:text-white

    hover:file:bg-slate-700
    `;

    return (
        <div class="relative space-y-1">
            <Label htmlFor={id} required={props["aria-required"] ? "true" : "false"}>
                {label}
            </Label>
            <input
                id={id}
                name={id}
                autocomplete="off"
                {...props}
                class={`
                    bg-transparent
                    border
                    border-slate-950
                    p-3
                    placeholder-slate-600
                    rounded
                    w-full

                    disabled:bg-slate-50
                    disabled:text-slate-500
                    disabled:border-slate-500

                    ${props.type === "file" ? fileStyles : ""}

                    peer

                    aria-[invalid=true]:border-red-500
                    aria-[invalid=true]:placeholder:text-red-500
                `}
            />

            <FieldError id={id} />
        </div>
    );
}
