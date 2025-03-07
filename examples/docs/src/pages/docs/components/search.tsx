export function Search() {
    return (
        <search-modal>
            <dialog
                id="search-modal"
                class="w-screen bg-transparent mt-6 mx-auto backdrop:bg-slate-900/50 backdrop:backdrop-blur-xs dark:backdrop:backdrop-blur-sm"
                $ref="dialog"
            >
                <div
                    class="flex flex-col mx-auto max-w-[640px] transform overflow-hidden rounded-xl border dark:border-slate-400"
                    role="document"
                >
                    <div class="p-2 relative z-10 bg-white dark:bg-slate-900 h-16 transition">
                        <div class="relative h-full">
                            <label for="search-input" class="sr-only">
                                Search documentation
                            </label>
                            <input
                                $ref="searchInput"
                                $bind-value="searchQuery"
                                class="peer rounded-xl border border-slate-200 dark:border-slate-400 bg-white dark:bg-slate-800 h-full w-full outline-none text-slate-900 dark:text-white placeholder:text-slate-400 tracking-tight pl-12 pr-14 focus:border-slate-900 dark:focus:border-slate-400 transition shadow-search"
                                id="search-input"
                                type="text"
                                placeholder="Search documentation..."
                                autocomplete="off"
                                role="combobox"
                                aria-expanded="true"
                                aria-autocomplete="list"
                                aria-controls="search-results-list"
                            />

                            <svg
                                class="absolute left-6 top-1/2 -translate-y-1/2 text-slate-900 dark:text-slate-400 opacity-50 peer-focus:opacity-100"
                                xmlns="http://www.w3.org/2000/svg"
                                width="18"
                                height="18"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                aria-hidden="true"
                            >
                                <circle cx="11" cy="11" r="8" />
                                <path d="m21 21-4.3-4.3" />
                            </svg>

                            <kbd class="absolute top-1/2 -translate-y-1/2 right-6 flex items-center justify-center rounded-md gap-1 px-1.5 py-1.5 text-slate-950/70 dark:text-white/70 font-medium text-xs leading-[9px]">
                                ESC
                            </kbd>
                        </div>
                    </div>

                    <ul
                        class="max-h-[648px] outline-transparent overflow-y-auto bg-white dark:bg-slate-900"
                        id="search-results-list"
                        $bind-html="searchResultsHtml"
                        tabindex="0"
                        role="listbox"
                        aria-label="Search Results"
                    />
                    <template id="search-result-template">
                        <li class="m-2">
                            <a
                                href="/"
                                role="option"
                                aria-selected="false"
                                tabindex="-1"
                                class="
                                    js-doc-link
                                    cursor-pointer
                                    relative
                                    rounded-xl
                                    flex
                                    gap-4
                                    px-4
                                    py-2
                                    items-center
                                    w-full
                                    text-left
                                    hover:bg-slate-100
                                    dark:hover:bg-white/5
                                    focus:outline-none
                                    focus:ring-2
                                    focus:ring-primary
                                    dark:focus:ring-white

                                    aria-selected:bg-slate-100
                                    dark:aria-selected:bg-white/5
                                    aria-selected:outline-none
                                    aria-selected:ring-2
                                    aria-selected:ring-primary
                                    dark:aria-selected:ring-white
                                "
                            >
                                <span class="flex flex-col w-[calc(100%-16px-20px)] leading-[26px] text-slate-700 dark:text-slate-300">
                                    <span class="js-doc-title text-base font-cal" />
                                    <span class="js-doc-section text-sm truncate" />
                                </span>
                            </a>
                        </li>
                    </template>
                </div>
            </dialog>
        </search-modal>
    );
}
