class SwitchComponent extends HTMLInputElement {
    connectedCallback() {
        this.addEventListener("change", this.handleChange);
    }

    handleChange(event: Event) {
        const input = event.target as HTMLInputElement;

        input.setAttribute("aria-checked", input.checked.toString());
    }
}

if (!customElements.get("switch-element")) {
    customElements.define("switch-element", SwitchComponent, { extends: "input" });
}
