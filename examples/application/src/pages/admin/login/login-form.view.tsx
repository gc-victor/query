import { CLASS_LOGIN_FORM_DESCRIPTION, CLASS_LOGIN_FORM_ERROR, CLASS_LOGIN_FORM_ERROR_TEXT, ID_LOGIN_FORM } from "./login.constants";

export function LoginFormView() {
    return (
        <form is="login-component" id={ID_LOGIN_FORM} class="mt-4 px-4" novalidate>
            <Input
                id="email"
                label="Email"
                autocomplete="email"
                description="Use the same email as on your server."
                placeholder="Your email address"
            />
            <Input
                id="password"
                label="Password"
                autocomplete="current-password"
                description="Use the same password as on your server."
                placeholder="********"
                type="password"
            />
            <button
                class={`
                    bg-slate-600
                    hover:bg-slate-700
                    focus:bg-slate-700
                    disabled:bg-slate-400
                    disabled:cursor-not-allowed
                    block
                    border
                    border-slate-700
                    h-12
                    mt-2
                    rounded-lg
                    text-white
                    w-full
                `}
                type="submit"
            >
                Login
            </button>
            <p class="mt-2 text-sm">
                The emails and passwords are the same as those you use on your server. Please note that you must be an admin to use all the
                available features.
            </p>
        </form>
    );
}

function Input({
    description,
    id,
    label,
    children,
    ...rest
}: {
    description: string;
    id: string;
    label: string;
    [key: string]: unknown;
}) {
    return (
        <p class="pb-6 relative">
            <label for={id} class="block font-bold w-full">
                {label}
            </label>
            <input
                id={id}
                class={`
                required:[&:placeholder-shown]:bg-white
                invalid:bg-red-100
                    border
                    border-slate-700
                    h-12
                    mt-2
                    px-4
                    rounded-lg
                    text-slate-600
                    w-full
                `}
                name={id}
                onClick={(e: Event) => (e.target as HTMLInputElement).setCustomValidity("")}
                {...rest}
            />
            <span
                class={`
                ${CLASS_LOGIN_FORM_DESCRIPTION}
                absolute
                block
                bottom-0
                text-sm
                text-slate-400
            `}
            >
                {description}
            </span>
            <span
                id={`err-${id}`}
                class={`
                ${CLASS_LOGIN_FORM_ERROR}
                absolute
                hidden
                bottom-0
                text-sm
                text-red-400
            `}
            >
                <span
                    class={`
                    w-2
                    h-2
                    bg-red-400
                    rounded-full
                    mr-1
                    inline-block
                `}
                />
                <span class={CLASS_LOGIN_FORM_ERROR_TEXT} />
            </span>
        </p>
    );
}
