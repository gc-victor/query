import { ID_DRAWER_COMPONENT, ID_FORM_COMPONENT, ID_NEW_ITEM } from "@/pages/admin/utils/constants";

import type { DrawerComponent } from "./drawer-element";
import type { FormComponent } from "./form-element";

class Table extends HTMLTableElement {
    connectedCallback() {
        this.addEventListener("click", this.handleClick);

        document.getElementById(ID_DRAWER_COMPONENT)?.addEventListener("close", async () => {
            if (this.dataset.url) {
                const response = await fetch(this.dataset.url);
                const html = await response.text();
                const template = document.createElement("template");
                template.innerHTML = html;
                const doc = template.content.cloneNode(true) as Document;
                const docTBody = doc.querySelector("tbody");

                if (docTBody) {
                    const tbody = this.querySelector("tbody");

                    tbody?.parentNode?.replaceChild(docTBody, tbody);
                }
            }
        });

        document.getElementById(ID_NEW_ITEM)?.addEventListener("click", async () => {
            (document.getElementById(ID_DRAWER_COMPONENT) as DrawerComponent).open = true;
        });
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

    handleClick(event: Event) {
        const target = event.target;

        if (target instanceof HTMLAnchorElement) {
            return;
        }

        event.preventDefault();

        const el = (target as HTMLElement).closest("tr");
        const uuid = el?.dataset.uuid;
        const drawer = document.getElementById(ID_DRAWER_COMPONENT) as DrawerComponent | null;
        const form = document.getElementById(ID_FORM_COMPONENT) as FormComponent;

        if (uuid && drawer && form) {
            form.dataset.uuid = uuid;
            drawer.open = true;
        }
    }
}

if (!customElements.get("table-element")) {
    customElements.define("table-element", Table, {
        extends: "table",
    });
}
