export const PageFooter = () => {
    return (
        <footer className="mx-auto w-full max-w-2xl space-y-10 pb-16 lg:max-w-5xl">
            <div className="flex flex-col items-center justify-between gap-3 border-t border-slate-900/5 pt-8 sm:flex-row dark:border-white/5">
                <p className="text-sm text-slate-600 dark:text-slate-400">
                    Generated with <span className="text-base font-cal">Q|ery.</span> Docs.
                </p>
                <div className="flex gap-4">
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
                </div>
            </div>
        </footer>
    );
};