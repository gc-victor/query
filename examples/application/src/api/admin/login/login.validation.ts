import { minLength, object, string, pipe } from "valibot";

export const LoginValidation = object({
    email: pipe(string(), minLength(1, "Please enter an email.")),
    password: pipe(string(), minLength(1, "Please enter a password."))
});
