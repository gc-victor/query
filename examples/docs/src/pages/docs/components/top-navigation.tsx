export const TopNavigation = () => {
    return (
        <div
            className="fixed inset-x-0 top-0 flex h-14 items-center justify-between gap-12 px-4 transition sm:px-6 lg:px-8 backdrop-blur-xs lg:left-72 xl:left-80 dark:backdrop-blur-sm bg-white/[var(--bg-opacity-light)] dark:bg-slate-900/[var(--bg-opacity-dark)]"
            style={{ "--bg-opacity-light": "0.5", "--bg-opacity-dark": "0.2" }}
        >
            <div className="absolute inset-x-0 top-full h-px transition bg-slate-900/7.5 dark:bg-white/7.5" />
            <div className="hidden items-center lg:flex lg:max-w-md lg:flex-auto">
                <svg className="w-[80px] h-[37px] hidden lg:block dark:text-white" role="banner">
                    <title>Q|ery.</title>
                    <use href="#icon-query-logo" />
                </svg>
                <button
                    id="search-button"
                    type="button"
                    class="hidden cursor-text h-8 w-full items-center gap-2 rounded-full ml-4 pr-3 pl-2 text-sm text-slate-400 ring-1 ring-slate-400 transition hover:ring-slate-500 hover:text-slate-500 lg:flex dark:bg-white/5 dark:text-slate-400 dark:ring-slate-400 dark:ring-inset dark:hover:ring-slate-500 dark:hover:text-slate-500"
                >
                    <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class="h-5 w-5 stroke-current">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M12.01 12a4.25 4.25 0 1 0-6.02-6 4.25 4.25 0 0 0 6.02 6Zm0 0 3.24 3.25"
                        />
                    </svg>
                    Find something...
                    <kbd class="ml-auto text-2xs">
                        <kbd class="font-sans">Ctrl</kbd> + <kbd class="font-sans">K</kbd>
                    </kbd>
                </button>
            </div>
            <div className="flex items-center gap-5 lg:hidden">
                <a aria-label="Home" href="/">
                    <svg className="w-[64px] h-[30px] dark:text-white" role="banner">
                        <title>Q|ery.</title>
                        <use href="#icon-query-logo" />
                    </svg>
                </a>
            </div>
            <div className="flex items-center gap-4">
                <div className="flex items-center gap-4">
                    <label
                        htmlFor="menu-toggle"
                        className="flex h-6 w-6 items-center justify-center rounded-md transition text-slate-900 dark:text-white hover:bg-slate-900/5 dark:hover:bg-white/5 lg:hidden"
                        aria-label="Toggle navigation"
                    >
                        <svg viewBox="0 0 16 16" fill="currentColor" className="size-4">
                            <title>Toggle navigation</title>
                            <path
                                fillRule="evenodd"
                                d="M2 4.75A.75.75 0 0 1 2.75 4h10.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 4.75Zm0 6.5a.75.75 0 0 1 .75-.75h10.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z"
                                clipRule="evenodd"
                            />
                        </svg>
                    </label>
                    <div class="contents lg:hidden">
                        <button
                            id="search-icon-button"
                            type="button"
                            class="flex h-6 w-6 items-center justify-center rounded-md transition hover:bg-slate-900/5 lg:hidden dark:hover:bg-white/5"
                            aria-label="Find something..."
                        >
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class="h-5 w-5 stroke-slate-900 dark:stroke-white">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M12.01 12a4.25 4.25 0 1 0-6.02-6 4.25 4.25 0 0 0 6.02 6Zm0 0 3.24 3.25"
                                />
                            </svg>
                        </button>
                    </div>
                    <button
                        id="themeToggle"
                        type="button"
                        className="flex h-6 w-6 items-center justify-center rounded-md transition hover:bg-slate-900/5 dark:hover:bg-white/5"
                        aria-label="Switch to dark theme"
                    >
                        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" className="h-5 w-5 stroke-slate-900 dark:hidden">
                            <path d="M12.5 10a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0Z" />
                            <path
                                strokeLinecap="round"
                                d="M10 5.5v-1M13.182 6.818l.707-.707M14.5 10h1M13.182 13.182l.707.707M10 15.5v-1M6.11 13.889l.708-.707M4.5 10h1M6.11 6.111l.708.707"
                            />
                        </svg>
                        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" className="hidden h-5 w-5 stroke-white dark:block">
                            <path d="M15.224 11.724a5.5 5.5 0 0 1-6.949-6.949 5.5 5.5 0 1 0 6.949 6.949Z" />
                        </svg>
                    </button>
                </div>
                <div className="hidden md:block md:h-5 md:w-px md:bg-slate-900/10 md:dark:bg-white/15" />
                <nav className="flex items-center gap-4">
                    <a className="group" href="https://github.com/gc-victor/query">
                        <span className="sr-only">Follow us on GitHub</span>
                        <svg
                            viewBox="0 0 20 20"
                            aria-hidden="true"
                            className="h-5 w-5 fill-slate-700 transition group-hover:fill-slate-900 dark:fill-white dark:group-hover:fill-slate-500"
                        >
                            <title>Follow us on GitHub</title>
                            <use href="#icon-github-logo" />
                        </svg>
                    </a>
                    <a className="group" href="https://qery.io/discord">
                        <span className="sr-only">Join our Discord server</span>
                        <svg
                            viewBox="0 0 20 20"
                            aria-hidden="true"
                            className="h-5 w-5 fill-slate-700 transition group-hover:fill-slate-900 dark:fill-white dark:group-hover:fill-slate-500"
                        >
                            <title>Join our Discord server</title>
                            <use href="#icon-discord-logo" />
                        </svg>
                    </a>
                </nav>
            </div>
        </div>
    );
};
