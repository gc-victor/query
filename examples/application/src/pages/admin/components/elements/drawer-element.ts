// @see: https://github.com/wes-goulet/side-drawer/blob/v4.2.0/side-drawer.js
const style: string = `
:host {
  background-color: #fff;
  width: 720px;
  max-width: 100vw;
  z-index: 10;
}

::slotted(*) {
  box-sizing: border-box;
}

dialog {
  all: unset;

  width: inherit;
  max-width: inherit;

  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  height: 100%;

  transform: translateX(-100%);
  transition: transform 0.25s ease-out;
  visibility: hidden;
}

:host([right]) dialog {
  left: unset;
  right: 0;
  transform: translateX(100%);
}

/* putting this here in case this is ever fixed:
 https://github.com/whatwg/html/issues/7732 */
dialog,
dialog::backdrop {
  overscroll-behavior: contain;
}

dialog:modal {
  background-color: inherit;
  box-shadow: 0px 0px 25px 0px rgba(0, 0, 0, 0.5);
  border-top-right-radius: inherit;
  border-bottom-right-radius: inherit;
  border-top-left-radius: inherit;
  border-bottom-left-radius: inherit;
}

dialog::backdrop {
  background-color: #000;
  backdrop-filter: none;
  opacity: 0;
  transition: opacity linear 0.25s;
}

dialog[open] {
  visibility: visible;
}

:host([open]) dialog[open],
:host([open]) dialog[open]::backdrop {
    transition-delay: 0s;
    transform: none;
}

:host([open]) dialog[open]::backdrop {
    transition-delay: 0s;
    opacity: .7;
}
`;

const template: string = `<dialog part="dialog"><slot></slot></dialog>`;
const tmpl: HTMLTemplateElement = document.createElement("template");
tmpl.innerHTML = `<style>${style}</style>${template}`;

export interface DrawerComponent extends HTMLElement {
    open: boolean;
    connectedCallback(): void;
}

class Drawer extends HTMLElement {
    #dialog: HTMLDialogElement;

    constructor() {
        super();

        const shadowRoot: ShadowRoot = this.attachShadow({ mode: "open" });
        shadowRoot.appendChild(tmpl.content.cloneNode(true));

        shadowRoot.host.classList.remove("hidden");

        this.#dialog = shadowRoot.querySelector("dialog") as HTMLDialogElement;
    }

    connectedCallback(): void {
        this.#dialog.addEventListener("click", (event: MouseEvent) => {
            if ((event.target as HTMLElement)?.nodeName === "DIALOG") {
                this.open = false;
            }
        });

        this.#dialog.addEventListener("close", () => {
            this.open = false;
        });

        this.upgradeProperty("open");
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

    get open(): boolean {
        return this.hasAttribute("open");
    }

    set open(isOpen: boolean) {
        if (isOpen && !this.hasAttribute("open")) {
            this.setAttribute("open", "");
        } else if (this.hasAttribute("open")) {
            this.removeAttribute("open");
        }
    }

    static get observedAttributes(): string[] {
        return ["open"];
    }

    attributeChangedCallback(name: string, oldValue: unknown, newValue: unknown): void {
        if (name === "open") {
            if (!this.open) {
                this.#dialog.addEventListener("transitionend", () => this.#dialog.close(), { once: true });

                this.dispatchEvent(
                    new CustomEvent("close", {
                        bubbles: true,
                    }),
                );
            } else {
                this.#dialog.showModal();

                this.dispatchEvent(
                    new CustomEvent("open", {
                        bubbles: true,
                    }),
                );
            }
        }
    }
}

if (!customElements.get("drawer-element")) {
    customElements.define("drawer-element", Drawer);
}
