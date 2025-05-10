import { ReactiveComponent } from "@qery/reactive-component";

export class CounterIsland extends ReactiveComponent {
    private timeoutId = 0;
    count!: number;

    connectedCallback(): void {
        super.connectedCallback();

        this.effect(() => {
            const count = this.count;

            this.storeCounterValue();
        });
    }

    increment() {
        this.count++;
    }

    decrement() {
        this.count--;
    }

    reset() {
        this.count = 0;
    }

    private storeCounterValue() {
        this.debounce(async () => {
            try {
                const response = await fetch("/api/counter", {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({ value: this.count }),
                });

                if (!response.ok) {
                    console.error("Failed to update counter value");
                }
            } catch (error) {
                console.error("Error storing counter value:", error);
            }
        }, 500)();
    }

    private debounce<T extends (...args: unknown[]) => unknown>(fn: T, delay: number): (...args: Parameters<T>) => void {
        return (...args: Parameters<T>) => {
            clearTimeout(this.timeoutId);
            this.timeoutId = window.setTimeout(() => fn(...args), delay);
        };
    }
}

customElements.define("counter-island", CounterIsland);
