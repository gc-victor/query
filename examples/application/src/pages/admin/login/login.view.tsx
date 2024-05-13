import { LoginFormView } from "./login-form.view";

export function LoginView() {
    return (
        <div class="flex items-center h-full justify-center">
            <div class="max-w-md md:w-[448px]">
                <h1 class="flex justify-center">
                    <svg height="64" width="173">
                        <title>Query Logo</title>
                        <use href="#query-logo" fill="rgb(255 255 255)" height="64" width="173" />
                    </svg>
                </h1>
                <LoginFormView />
            </div>
        </div>
    );
}
