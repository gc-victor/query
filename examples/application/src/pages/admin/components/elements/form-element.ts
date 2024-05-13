import { CLASS_FORM_ERROR_TEXT, ID_DRAWER_COMPONENT } from "@/pages/admin/utils/constants";

import type { DrawerComponent } from "./drawer-element";

export interface FormComponent extends HTMLElement {
    setFormData(): Promise<void>;
}

class Form extends HTMLFormElement {
    connectedCallback() {
        document.getElementById(ID_DRAWER_COMPONENT)?.addEventListener("close", () => {
            this.removeAttribute("data-uuid");
            this.reset();
        });

        this.addEventListener("reset", this.close);

        this.inputDate();

        for (const button of Array.from(this.querySelectorAll('button[type="submit"]'))) {
            button.addEventListener("click", this.handleSubmit.bind(this));
        }
    }

    [key: string]: unknown;

    private inputDate() {
        const showPicker = (event: Event) => {
            if ((event as KeyboardEvent).type === "keydown" && !["Tab", "Escape"].includes((event as KeyboardEvent).key)) {
                event.preventDefault();
                (event.target as HTMLInputElement).showPicker();
            }
        };

        for (const el of Array.from(this.querySelectorAll('input[type="date"]'))) {
            el.addEventListener("click", showPicker);
            el.addEventListener("focus", showPicker);
            el.addEventListener("keydown", showPicker);
        }
    }

    // CREDIT: https://web.dev/custom-elements-best-practices/#make-properties-lazy
    upgradeProperty(prop: string): void {
        if (Object.prototype.hasOwnProperty.call(this, prop)) {
            const value: unknown = this[prop];
            delete this[prop];
            this[prop] = value;
        }
    }

    static get observedAttributes(): string[] {
        return ["data-uuid"];
    }

    async attributeChangedCallback(name: string, oldValue: unknown, newValue: unknown) {
        if (name === "data-uuid" && newValue) {
            await this.setFormData();
        }
    }

    async setFormData() {
        const res = await fetch(`${this.dataset.path}/uuid/${this.dataset.uuid}`, { method: "GET" });
        const json = await res.json();
        const jsonData = json.data[0];

        for (const fieldName of Object.keys(jsonData)) {
            const fieldValue = jsonData[fieldName];
            const formField = this.querySelectorAll(`[name="${fieldName}"]`) as NodeListOf<
                HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement
            >;

            if (formField.length) {
                for (const el of Array.from(formField)) {
                    if (el.tagName === "INPUT" && el.type === "checkbox") {
                        (el as HTMLInputElement).checked = fieldValue;
                    } else if (el.tagName === "INPUT" && el.type === "date") {
                        (el as HTMLInputElement).valueAsNumber = fieldValue * 1000;
                    } else {
                        el.value = fieldValue;
                    }
                }
            }
        }
    }

    async handleSubmit(e: Event) {
        e.preventDefault();

        if ((e.target as HTMLButtonElement)?.getAttribute("formmethod") === "delete") {
            this.delete(e);
        } else if (this.dataset.uuid) {
            this.update(e);
        } else {
            this.create(e);
        }
    }

    async delete(e: Event) {
        e.preventDefault();

        if (this.dataset.uuid) {
            await fetch(`${this.dataset.path}`, { method: "DELETE", body: JSON.stringify({ uuid: this.dataset.uuid }) });
        }

        this.close();
    }

    async create(e: Event) {
        e.preventDefault();

        const formData = new FormData(this);

        // NOTE: Workaround to send binary data as it fails in Query as it isn't a valid UTF-8 string
        await this.fileToData(formData);
        // NOTE: it forces to set the formData value to valueAsNumber
        this.dateToValueAsNumber(formData);

        const res = await fetch(`${this.dataset.path}`, { method: "POST", body: formData });

        if (res.ok) {
            this.close();
        } else {
            if (res.status < 500) {
                const json = await res.json();

                this.setFieldErrors(json.errors);
            }
        }
    }

    async update(e: Event) {
        e.preventDefault();

        const formData = new FormData(this);

        // NOTE: Workaround to send binary data as it fails in Query as it isn't a valid UTF-8 string
        await this.fileToData(formData);
        // NOTE: it forces to set the formData value to valueAsNumber
        this.dateToValueAsNumber(formData);

        if (this.dataset.uuid) {
            formData.set("uuid", this.dataset.uuid);
        }

        const res = await fetch(`${this.dataset.path}`, { method: "PUT", body: formData });

        if (res.ok) {
            this.close();
        } else {
            if (res.status < 500) {
                const json = await res.json();

                this.setFieldErrors(json.errors);
            }
        }
    }

    close() {
        const drawer = document.getElementById(ID_DRAWER_COMPONENT) as DrawerComponent;

        if (drawer) {
            drawer.open = false;

            for (const el of Array.from(this.querySelectorAll("input, textarea, select"))) {
                this.updateField(el as HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement, true, "");
            }
        }
    }

    setFieldErrors(errors: Record<string, string[]>) {
        for (const fieldName of Object.keys(errors)) {
            const fieldError = errors[fieldName][0];
            const formField = this.querySelector(`[name="${fieldName}"]`) as HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;

            if (fieldError) {
                this.updateField(formField, false, fieldError);
            } else {
                this.updateField(formField, true, "");
            }
        }
    }

    updateField(el: HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement, isValid: boolean, errorMessage: string) {
        const parentElement = el.parentElement as HTMLElement;
        const elError = parentElement?.querySelector(`.${CLASS_FORM_ERROR_TEXT}`) as HTMLElement;

        if (isValid) {
            el.removeAttribute("aria-invalid");
            el.removeAttribute("aria-describedby");
            el.setCustomValidity("");
            el.reportValidity();

            parentElement.classList.remove("text-red-500");

            if (elError) {
                elError.textContent = "";
            }

            el.removeEventListener("input", this.clearValidation.bind(this));
        } else {
            const id = el.id;

            el.setAttribute("aria-invalid", "true");
            el.setAttribute("aria-describedby", `err-${id}`);
            el.setCustomValidity(errorMessage);
            el.reportValidity();

            parentElement.classList.add("text-red-500");

            if (elError) {
                elError.setAttribute("aria-live", "assertive");
                elError.textContent = errorMessage;
            }

            if (el.type !== "date") {
                el.addEventListener("input", this.clearValidation.bind(this));
            }

            // NOTE: avoids the native tooltip
            (this.querySelector('button[type="submit"]') as HTMLButtonElement)?.focus();
        }
    }

    clearValidation(e: Event) {
        const event = e as KeyboardEvent;

        this.updateField(event.target as HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement, true, "");
    }

    // NOTE: Workaround to send binary data as it fails in Query as it isn't a valid UTF-8 string
    private async fileToData(formData: FormData) {
        const formDataEntries: { key: string; value: FormDataEntryValue }[] = [];

        formData.forEach((value, key) => formDataEntries.push({ key, value }));

        for (const { key, value } of formDataEntries) {
            if (value instanceof File) {
                const arrayBuffer = await value.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);
                formData.set(key, new Blob([JSON.stringify(Array.from(uint8Array))], { type: value.type }), value.name);
            }
        }
    }

    private dateToValueAsNumber(formData: FormData) {
        for (const el of Array.from(this.querySelectorAll('input[type="date"]'))) {
            const inputElement = el as HTMLInputElement;
            formData.set(inputElement.name, (inputElement.valueAsNumber / 1000).toString());
        }
    }
}

if (!customElements.get("form-element")) {
    customElements.define("form-element", Form, { extends: "form" });
}
