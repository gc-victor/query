import { CLASS_FORM_ERROR_TEXT } from "@/pages/admin/utils/constants";

export function FieldError({ id }: { id: string }) {
    return (
        <span
            class={`
                absolute
                -bottom-[1.15rem]
                hidden
                text-sm
                text-red-500

                peer-aria-[invalid=true]:block
            `}
        >
            <span
                class={`
                    w-2
                    h-2
                    bg-red-500
                    rounded-full
                    mr-1
                    inline-block
                `}
            />
            <span id={`err-${id}`} class={CLASS_FORM_ERROR_TEXT} />
        </span>
    );
}
