import type { Toc, Navigation } from "../types";

export const LeftNavigation = ({ toc, navigation }: { toc: Toc; navigation: Navigation | null | undefined }) => {
    return (
        <aside className="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
            <div className="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-slate-900/10 lg:px-6 lg:py-4 lg:pb-8 xl:w-80 dark:lg:border-white/10">
                <input type="checkbox" id="menu-toggle" className="hidden peer" />
                <nav className="fixed inset-0 -translate-x-full w-72 max-w-[80%] z-50 bg-white dark:bg-slate-900 p-4 h-full overflow-y-auto transition-transform duration-300 ease-in-out peer-checked:translate-x-0 lg:relative lg:transform-none lg:translate-x-0 lg:p-0 lg:h-auto lg:z-0 lg:overflow-visible">
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
                            <ul className="toc-list space-y-4 text-slate-900 dark:text-white">
                                {toc?.items?.map((group, groupIndex) => (
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
                                                        className={`block text-lg lg:text-sm level-${item.level}`}
                                                        aria-current={item.url === navigation?.current?.url ? "page" : undefined}
                                                    >
                                                        {item.title}
                                                    </a>
                                                    {item.children && (
                                                        <ul>
                                                            {item.children.map((child, childIndex) => (
                                                                <li
                                                                    className={
                                                                        child.url === navigation?.current?.url ? "active font-semibold" : ""
                                                                    }
                                                                >
                                                                    <a
                                                                        href={child.url}
                                                                        className={`block text-sm level-${child.level}`}
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
                {/* Backdrop overlay for mobile */}
                <label
                    htmlFor="menu-toggle"
                    className="fixed inset-0 bg-slate-900/50 dark:bg-white/10 backdrop-blur-sm z-40 hidden peer-checked:block lg:hidden"
                    aria-hidden="true"
                />
            </div>
        </aside>
    );
};
