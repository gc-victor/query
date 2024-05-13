import { API_ADMIN_LOGIN_PATH, PAGE_ADMIN_PATH } from "@/config/shared/shared.constants";
import { CLASS_LOGIN_FORM_DESCRIPTION, CLASS_LOGIN_FORM_ERROR, CLASS_LOGIN_FORM_ERROR_TEXT } from "./login.constants";

class Form extends HTMLFormElement {
    connectedCallback() {
        this.addEventListener("submit", this.handleSubmit.bind(this));
        this.addEventListener("input", this.handleInput.bind(this));
    }

    [key: string]: unknown;

    // from https://web.dev/custom-elements-best-practices/#make-properties-lazy
    upgradeProperty(prop: string): void {
        if (Object.prototype.hasOwnProperty.call(this, prop)) {
            const value: unknown = this[prop];
            delete this[prop];
            this[prop] = value;
        }
    }

    get error(): string {
        return this.getAttribute("error") as string;
    }

    set error(value: string) {
        if (value) {
            this.setAttribute("error", value);
        } else if (this.hasAttribute("error")) {
            this.removeAttribute("error");
        }
    }

    get path(): string {
        return this.getAttribute("path") as string;
    }

    set path(value: string) {
        if (value) {
            this.setAttribute("path", value);
        } else {
            if (this.hasAttribute("path")) {
                this.removeAttribute("path");
            }
        }
    }

    static get observedAttributes(): string[] {
        return ["error"];
    }

    async attributeChangedCallback(name: string, oldValue: unknown, newValue: unknown) {
        const emailInput = this.querySelector('input[name="email"]') as HTMLInputElement;
        const passwordInput = this.querySelector('input[name="password"]') as HTMLInputElement;

        if (name === "error" && newValue) {
            for (const el of Array.from(this.querySelectorAll(`.${CLASS_LOGIN_FORM_DESCRIPTION}`))) {
                el.classList.add("hidden");
                el.classList.remove("block");
            }

            for (const el of Array.from(this.querySelectorAll(`.${CLASS_LOGIN_FORM_ERROR}`))) {
                el.classList.remove("hidden");
                el.classList.add("block");

                const text = el.querySelector(`.${CLASS_LOGIN_FORM_ERROR_TEXT}`) as HTMLElement;
                text.textContent = newValue as string;
            }

            emailInput.setAttribute("aria-invalid", "true");
            emailInput.setAttribute("aria-errormessage", "err-email");
            emailInput.setCustomValidity(newValue as string);
            emailInput.reportValidity();

            passwordInput.setAttribute("aria-invalid", "true");
            passwordInput.setAttribute("aria-errormessage", "err-email");
            passwordInput.setCustomValidity(newValue as string);
            passwordInput.reportValidity();

            // NOTE: avoids the native tooltip
            this.querySelector("button")?.focus();
        } else if (name === "error") {
            for (const el of Array.from(this.querySelectorAll(`.${CLASS_LOGIN_FORM_DESCRIPTION}`))) {
                el.classList.remove("hidden");
                el.classList.add("block");
            }

            for (const el of Array.from(this.querySelectorAll(`.${CLASS_LOGIN_FORM_ERROR}`))) {
                el.classList.add("hidden");
                el.classList.remove("block");

                const text = el.querySelector(`.${CLASS_LOGIN_FORM_ERROR_TEXT}`) as HTMLElement;
                text.textContent = "";
            }

            emailInput.removeAttribute("aria-invalid");
            emailInput.removeAttribute("aria-errormessage");
            emailInput.setCustomValidity("");
            emailInput.reportValidity();

            passwordInput.removeAttribute("aria-invalid");
            passwordInput.removeAttribute("aria-errormessage");
            passwordInput.setCustomValidity("");
            passwordInput.reportValidity();
        }
    }

    async handleSubmit(e: Event) {
        e.preventDefault();

        const res = await fetch(API_ADMIN_LOGIN_PATH, {
            method: "POST",
            body: new FormData(e.target as HTMLFormElement),
        });

        if (res.ok) {
            window.location.href = PAGE_ADMIN_PATH;
        } else {
            this.error = await res.text();
        }
    }

    handleInput() {
        this.error = "";
    }
}

if (!customElements.get("login-component")) {
    customElements.define("login-component", Form, { extends: "form" });
}
