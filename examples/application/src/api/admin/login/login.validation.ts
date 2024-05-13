import { minLength, object, string } from "valibot";

export const LoginValidation = object({
    email: string([minLength(1, "Please enter a email.")]),
    password: string([minLength(1, "Please enter a password.")]),
});
