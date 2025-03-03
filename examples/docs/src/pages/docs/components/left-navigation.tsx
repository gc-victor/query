import type { Toc, Navigation } from "../types";


export const LeftNavigation = ({ toc, navigation }: { toc: Toc; navigation: Navigation | null | undefined }) => {
    return (
        <header className="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
            <div className="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-slate-900/10 lg:px-6 lg:pt-4 lg:pb-8 xl:w-80 lg:dark:border-white/10">
                <input type="checkbox" id="menu-toggle" className="hidden" />
                <nav className="hidden w-full z-50 bg-white dark:bg-slate-900 lg:relative p-4 lg:p-0 h-full lg:h-auto overflow-y-scroll lg:z-0 lg:overflow-visible lg:block">
                    <label
                        htmlFor="menu-toggle"
                        className="fixed top-4 right-4 w-8 h-8 flex items-center justify-center text-slate-900 dark:text-white hover:text-slate-700 cursor-pointer lg:hidden"
                    >
                        <span className="sr-only">Close menu</span>
                        <svg
                            className="w-6 h-6"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                            aria-hidden="true"
                        >
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </label>
                    <h2 className="font-cal mb-8 pb-4 border-b border-slate-200 text-slate-900 text-xl lg:text-base dark:border-slate-700 dark:text-white">
                        Table of Contents
                    </h2>
                    <div className="relative">
                        <nav className="toc" aria-label="Table of Contents">
                            <ul className="toc-list space-y-4 dark:text-white">
                                {toc?.items?.map((group) => (
                                    <li className="pb-4 border-b border-slate-200 space-y-2 dark:border-slate-700">
                                        <h3 className="font-cal text-lg lg:text-sm">{group.name}</h3>
                                        <ul className="border-l border-slate-400 pl-4">
                                            {group.items?.map((item, itemIndex) => (
                                                <li
                                                    className={`toc-item${
                                                        item.url === navigation?.current?.url ? " active font-semibold" : ""
                                                    }`}
                                                >
                                                    <a
                                                        href={item.url}
                                                        className={`text-lg lg:text-sm level-${item.level}`}
                                                        aria-current={item.url === navigation?.current?.url ? "page" : undefined}
                                                    >
                                                        {item.title}
                                                    </a>
                                                    {item.children && (
                                                        <ul>
                                                            {item.children.map((child, childIndex) => (
                                                                <li className={child.url === navigation?.current?.url ? "active" : ""}>
                                                                    <a
                                                                        href={child.url}
                                                                        className={`lg:text-sm level-${child.level}`}
                                                                        aria-current={
                                                                            child.url === navigation?.current?.url ? "page" : undefined
                                                                        }
                                                                    >
                                                                        {child.title}
                                                                    </a>
                                                                </li>
                                                            ))}
                                                        </ul>
                                                    )}
                                                </li>
                                            ))}
                                        </ul>
                                    </li>
                                ))}
                            </ul>
                        </nav>
                    </div>
                </nav>
            </div>
        </header>
    );
};
