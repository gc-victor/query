class CounterIsland extends HTMLElement {
    private count = 0;
    private counterElement: HTMLElement | null = null;
    private timeoutId: number;

    constructor() {
        super();

        this.timeoutId = 0;
        this.counterElement = this.querySelector("[data-counter]");
    }

    async connectedCallback() {
        this.count = Number(this.counterElement?.textContent || 0);
        this.updateDisplay();

        this.addEventListener("click", this.handleClick.bind(this));
    }

    private async handleClick(e: Event) {
        e.preventDefault();

        const target = e.target as HTMLElement;

        const closestActionElement = target.closest("[data-action]") as HTMLElement;
        if (!closestActionElement) return;

        const action = closestActionElement.dataset.action;
        if (!action) return;

        if (action === "increment") {
            this.count++;
        } else if (action === "decrement") {
            this.count--;
        } else if (action === "reset") {
            this.count = 0;
        } else {
            return;
        }

        this.updateDisplay();
        this.storeCounterValue();
    }

    private storeCounterValue() {
        // Update counter in database
        return this.debounce(async () => {
            await fetch("/api/counter", {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ value: this.count }),
            });
        }, 500)();
    }

    private updateDisplay() {
        this.counterElement = this.querySelector("[data-counter]");
        if (this.counterElement) {
            this.counterElement.textContent = this.count.toString();
        }
    }

    private debounce<T extends (...args: unknown[]) => void>(fn: T, delay: number): (...args: Parameters<T>) => void {
        return (...args: Parameters<T>) => {
            clearTimeout(this.timeoutId);
            this.timeoutId = setTimeout(() => fn(...args), delay);
        };
    }
}

customElements.define("counter-island", CounterIsland);
