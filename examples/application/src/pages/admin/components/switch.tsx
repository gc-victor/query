import { Label } from "./label";

type Checked = "true" | "false";

export function Switch({
    label,
    id,
    checked = "false",
    ...props
}: { label: string; id: string; checked?: Checked; [key: string]: string | number | boolean | undefined; }) {
    return (
        <div className="flex flex-col">
            <span className="w-0">
                <Label htmlFor={id}>
                    <span class="inline-block mb-2">{label}</span>
                    <span className="inline-block text-base h-6 w-12">
                        <span class="text-base h-6 relative w-12 m-0">
                            <input
                                is="switch-element"
                                id={id}
                                name={id}
                                class="peer cursor-pointer h-full opacity-0 absolute w-full z-0 m-0 left-0 top-0"
                                type="checkbox"
                                role="switch"
                                aria-checked={checked}
                                checked={checked === "true"}
                                {...props}
                            />
                            <span
                                class="
                                        bg-slate-300
                                        box-border
                                        block
                                        h-full
                                        relative
                                        transition-all
                                        duration-300
                                        ease-[ease-in-out]
                                        w-full
                                        z-[-1]
                                        rounded-[3rem]

                                        after:bg-white
                                        after:content-['']
                                        after:h-4
                                        after:absolute
                                        after:translate-x-0
                                        after:transition-all
                                        after:duration-300
                                        after:ease-[ease-in-out]
                                        after:w-4
                                        after:rounded-[50%]
                                        after:left-1
                                        after:top-1

                                        after:peer-checked:content-['']
                                        after:peer-checked:translate-x-6

                                        peer-focus:outline-2
                                        peer-focus:ring-2
                                        peer-focus:ring-slate-950

                                        peer-hover:bg-slate-400
                                        peer-checked:bg-slate-950
                                        peer-checked:peer-hover:bg-slate-400
                                    "
                            />
                        </span>
                    </span>
                </Label>
            </span>
        </div>
    );
}
