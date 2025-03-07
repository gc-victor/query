import type { Navigation, Toc } from "@/pages/docs/types";
import { LeftNavigation } from "./left-navigation";
import { PageFooter } from "./page-footer";
import { Search } from "./search";
import { TopNavigation } from "./top-navigation";

export const DocumentTemplate = ({
    content,
    navigation,
    toc,
}: { content: JSX.Element | string; navigation: Navigation | null | undefined; toc: Toc }) => {
    return (
        <div className="w-full">
            <div className="h-full lg:ml-72 xl:ml-80">
                <LeftNavigation toc={toc} navigation={navigation} />

                <Search />
                <div className="relative flex h-full flex-col px-4 pt-14 sm:px-6 lg:px-8">
                    <TopNavigation />
                    <main className="flex-auto">
                        <article className="flex h-full flex-col pt-8 pb-10 md:pt-10 lg:pt-16">
                            <div className="flex-auto prose dark:prose-invert">
                                <div className="content">{content}</div>

                                <footer className="mx-auto mt-16 w-full max-w-2xl lg:max-w-5xl">
                                    <div className="mb-12 px-0.5 flex items-center font-semibold text-slate-700 dark:text-slate-200">
                                        {navigation?.previous ? (
                                            <a className="flex items-center space-x-3 group" href={navigation.previous.url}>
                                                <svg
                                                    viewBox="0 0 3 6"
                                                    className="h-1.5 stroke-slate-900 overflow-visible group-hover:stroke-slate-600 dark:stroke-white dark:group-hover:stroke-slate-300"
                                                    aria-hidden="true"
                                                >
                                                    <path
                                                        d="M3 0L0 3L3 6"
                                                        fill="none"
                                                        strokeWidth="2"
                                                        strokeLinecap="round"
                                                        strokeLinejoin="round"
                                                    />
                                                </svg>
                                                <span className="group-hover:text-slate-900 dark:group-hover:text-white">
                                                    {navigation.previous.title}
                                                </span>
                                            </a>
                                        ) : (
                                            ""
                                        )}

                                        {navigation?.next ? (
                                            <a className="flex items-center ml-auto space-x-3 group" href={navigation.next.url}>
                                                <span className="group-hover:text-slate-900 dark:group-hover:text-white">
                                                    {navigation.next.title}
                                                </span>
                                                <svg
                                                    viewBox="0 0 3 6"
                                                    className="rotate-180 h-1.5 stroke-slate-900 overflow-visible group-hover:stroke-slate-600 dark:stroke-white dark:group-hover:stroke-slate-300"
                                                    aria-hidden="true"
                                                >
                                                    <path
                                                        d="M3 0L0 3L3 6"
                                                        fill="none"
                                                        strokeWidth="2"
                                                        strokeLinecap="round"
                                                        strokeLinejoin="round"
                                                    />
                                                </svg>
                                            </a>
                                        ) : (
                                            ""
                                        )}
                                    </div>
                                </footer>
                            </div>
                        </article>
                    </main>

                    <PageFooter />
                </div>
            </div>
        </div>
    );
};
