export function Counter({ count }: { count: number }) {
    return (
        <>
            <div className="flex items-center justify-center text-4xl font-bold text-slate-800 dark:text-slate-200 mb-6">
                <button
                    className="p-2 rounded-full hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                    type="button"
                    data-action="decrement"
                >
                    <svg
                        className="w-6 h-6"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    >
                        <title>Minus</title>
                        <path d="M5 12h14" />
                    </svg>
                </button>
                <span className="mx-4 w-12" data-counter>{count}</span>
                <button
                    className="p-2 rounded-full hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                    type="button"
                    data-action="increment"
                >
                    <svg
                        className="w-6 h-6"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    >
                        <title>Plus</title>
                        <path d="M5 12h14" />
                        <path d="M12 5v14" />
                    </svg>
                </button>
            </div>
            <div className="flex justify-center">
                <button
                    class="
                        inline-flex
                        items-center
                        justify-center
                        whitespace-nowrap
                        rounded-md
                        text-sm
                        font-medium
                        ring-offset-background
                        transition-colors
                        focus-visible:outline-none
                        focus-visible:ring-2
                        focus-visible:ring-ring
                        focus-visible:ring-offset-2
                        disabled:pointer-events-none
                        disabled:opacity-50
                        border
                        border-input
                        bg-background
                        hover:bg-accent
                        hover:text-accent-foreground
                        h-10
                        px-4
                        py-2
                        mr-2"
                    type="reset"
                    data-action="reset"
                >
                    Reset
                </button>
            </div>
        </>
    );
}
