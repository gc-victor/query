import { ReactiveComponent } from "@qery/reactive-component";

interface SearchResult {
    title: string;
    section: string;
    path: string;
}

class SearchModal extends ReactiveComponent {
    private searchResults: SearchResult[] = [];
    private selectedIndex = -1;
    private typeaheadBuffer = "";
    private typeaheadTimeout: number | null = null;

    constructor() {
        super();

        this.initializeState();
        this.setupComputedProperties();
    }

    connectedCallback(): void {
        super.connectedCallback();

        this.bindEventListeners();
        this.attachSearchHandlers();
    }

    private initializeState(): void {
        this.setState("searchResults", []);
        this.setState("selectedIndex", -1);
    }

    private setupComputedProperties(): void {
        this.compute("searchResultsHtml", ["searchResults"], (...args: unknown[]) => {
            const [results] = args as [SearchResult[], number];
            return this.generateSearchResultsHtml(results);
        });
    }

    private generateSearchResultsHtml(results: SearchResult[]): string {
        return results
            .map((result, index) => {
                const template = this.querySelector("#search-result-template") as HTMLTemplateElement;
                const resultElement = template.content.cloneNode(true) as DocumentFragment;

                const pathElement = resultElement.querySelector(".js-doc-link") as HTMLAnchorElement;
                const titleElement = resultElement.querySelector(".js-doc-title");
                const sectionElement = resultElement.querySelector(".js-doc-section");

                if (pathElement) {
                    pathElement.href = result.path;
                    pathElement.setAttribute("aria-selected", (index === this.selectedIndex).toString());
                }
                if (titleElement) titleElement.textContent = result.title;
                if (sectionElement) sectionElement.innerHTML = result.section;

                return resultElement.querySelector("li")?.outerHTML || "";
            })
            .join("");
    }

    private bindEventListeners(): void {
        document.addEventListener("keydown", (e: KeyboardEvent) => this.handleKeyNavigation(e));

        const openIconButton = document.getElementById("search-icon-button");
        const openButton = document.getElementById("search-button");
        const dialog = this.refs?.dialog as HTMLDialogElement;

        openButton?.addEventListener("click", this.handleOpen);
        openIconButton?.addEventListener("click", this.handleOpen);

        dialog.addEventListener("close", () => {
            dialog.removeEventListener("click", this.handleOutsideClick);
            (this.refs.searchInput as HTMLInputElement).value = "";
            this.setState("searchResults", []);
        });

        // Add click listener for search results
        this.addEventListener("click", (e: Event) => {
            const target = e.target as HTMLElement;
            const optionElement = target.closest('a[role="option"]');
            if (optionElement) {
                e.preventDefault();
                const allOptions = Array.from(this.querySelectorAll('a[role="option"]'));
                const clickedIndex = allOptions.indexOf(optionElement as HTMLElement);
                if (clickedIndex !== -1) {
                    this.setState("selectedIndex", clickedIndex);
                    this.selectResult(clickedIndex);
                }
            }
        });
    }

    private handleOpen = () => {
        const dialog = this.refs?.dialog as HTMLDialogElement;
        dialog?.showModal();
        const searchInput = this.refs.searchInput as HTMLInputElement;
        searchInput?.focus();
        dialog.addEventListener("click", this.handleOutsideClick);
    };

    private handleOutsideClick = (event: MouseEvent): void => {
        const dialog = this.refs?.dialog as HTMLDialogElement;
        if (event.target === dialog) {
            dialog.close();
        }
    };

    private async performSearch(query: string): Promise<void> {
        this.setState("isLoading", true);

        try {
            const results = await this.searchContent(query);
            this.setState("searchResults", results);
            this.setState("selectedIndex", results.length > 0 ? 0 : -1);
            this.searchResults = results;
        } finally {
            this.setState("isLoading", false);
        }
    }

    private async searchContent(query: string): Promise<SearchResult[]> {
        if (!query.trim()) {
            return [];
        }

        try {
            const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
            if (!response.ok) {
                throw new Error("Search request failed");
            }
            const results = await response.json();

            return results.results;
        } catch (error) {
            console.error("Search error:", error);
            return [];
        }
    }

    private handleKeyNavigation(event: KeyboardEvent): void {
        const { key } = event;
        
        if ((event.key === "k" || event.key === "K") && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            this.handleOpen();
        }

        if (this.searchResults.length > 0) {
            const keyHandlers: Record<string, () => number> = {
                ArrowUp: () => (this.selectedIndex - 1 + this.searchResults.length) % this.searchResults.length,
                ArrowDown: () => (this.selectedIndex + 1) % this.searchResults.length,
                Home: () => 0,
                End: () => this.searchResults.length - 1,
            };

            const navigationKey = keyHandlers[key];
            if (navigationKey) {
                event.preventDefault();
                const newIndex = navigationKey();
                this.setState("selectedIndex", newIndex);
                this.scrollSelectedIntoView(newIndex);
            } else if (key === "Enter" && this.selectedIndex >= 0) {
                event.preventDefault();
                this.selectResult(this.selectedIndex);
            } else if (key.length === 1 && !event.ctrlKey && !event.altKey && !event.metaKey) {
                const searchInput = this.refs.searchInput as HTMLInputElement;
                if (searchInput !== document.activeElement) {
                    event.preventDefault();
                    this.handleTypeahead(key);
                }
            }
        }
    }

    private attachSearchHandlers(): void {
        const searchInput = this.refs.searchInput as HTMLInputElement;
        searchInput?.addEventListener("input", this.debounce(this.handleInput, 300));
    }

    private async handleInput(e: unknown) {
        const value = (e as Event).target as HTMLInputElement;
        if (value.value.length > 2) {
            await this.performSearch(value.value);
        }
    }

    private debounce<T extends (...args: unknown[]) => void>(func: T, delay: number): T {
        let timeoutId: number | null = null;

        return ((...args: Parameters<T>) => {
            if (timeoutId) {
                clearTimeout(timeoutId);
            }

            timeoutId = setTimeout(() => {
                func.apply(this, args);
                timeoutId = null;
            }, delay);
        }) as T;
    }

    private scrollSelectedIntoView(index: number): void {
        const resultElements = Array.from(this.querySelectorAll('[role="option"]'));
        const selectedElement = resultElements[index] as HTMLElement;

        if (selectedElement) {
            for (const el of resultElements) {
                el.setAttribute("aria-selected", "false");
                el.setAttribute("tabindex", "-1"); 
            }
            selectedElement.setAttribute("aria-selected", "true");
            selectedElement.setAttribute("tabindex", "0");
            selectedElement.focus();
            selectedElement.scrollIntoView({
                block: "nearest",
                behavior: "smooth",
            });
        }
    }

    private handleTypeahead(char: string): void {
        if (this.typeaheadTimeout) {
            clearTimeout(this.typeaheadTimeout);
        }

        this.typeaheadBuffer += char.toLowerCase();

        const matchIndex = this.searchResults.findIndex((result) => result.title.toLowerCase().startsWith(this.typeaheadBuffer));

        if (matchIndex !== -1) {
            this.setState("selectedIndex", matchIndex);
            this.scrollSelectedIntoView(matchIndex);
        }

        this.typeaheadTimeout = window.setTimeout(() => {
            this.typeaheadBuffer = "";
            this.typeaheadTimeout = null;
        }, 500) as unknown as number;
    }

    private selectResult(index: number): void {
        if (index >= 0 && index < this.searchResults.length) {
            const result = this.searchResults[index];
            window.location.href = result.path;
        }
    }
}

customElements.define("search-modal", SearchModal);
