const IS_DEVELOPMENT = process.env.QUERY_APP_ENV === "development";

const js = String.raw;

export function HotReload({ href }: { href: string }) {
    return IS_DEVELOPMENT ? (
        <script data-hot-reload="true">
            ${js`
                console.log("Hot reload running ...");

                const originalDefine = customElements.define;

                customElements.define = function(tagName, elementClass, options = {}) {
                    console.log('[HotReload] Defining custom element:', tagName);
                    const tempName = String(tagName) + "-" + Date.now();

                    let elements = [...document.querySelectorAll(tagName)];
                    let newElements = [];

                    if (elements.length) {
                        console.log('[HotReload] Found', elements.length, 'instances of', tagName, 'to update');
                        elements.forEach(oldElement => {
                            const newElement = document.createElement(tempName);

                            if (!oldElement.id) {
                                const innerHTML = oldElement.innerHTML;
                                oldElement.dataset.hrc = innerHTML.split('').reduce((out, i) => (101 * out + i.charCodeAt(0)) >>> 0, 11);
                                console.log('[HotReload] Generated HRC hash for element:', tagName, oldElement.dataset.hrc);
                            }

                            [...oldElement.attributes].forEach(attr => {
                                newElement.setAttribute(attr.name, attr.value);
                            });

                            newElement.append(...oldElement.childNodes);
                            oldElement.parentNode?.replaceChild(newElement, oldElement);
                            newElements.push(newElement);
                        });

                        originalDefine.call(customElements, tempName, elementClass, options);

                        newElements.forEach(element => restoreReactiveComponentsState(element));
                        console.log('[HotReload] Successfully updated', newElements, 'elements');
                    } else {
                        console.log('[HotReload] No existing instances found for', tagName);
                    }
                };

                function getElementPosition(element) {
                    const path = [];
                    let current = element;

                    while (current.parentElement) {
                        const index = Array.from(current.parentElement.children)
                            .filter(child => child.tagName === current.tagName)
                            .indexOf(current);

                        path.unshift({
                            tag: current.tagName.toLowerCase(),
                            index
                        });

                        current = current.parentElement;
                    }

                    return path;
                }

                let timeoutId;
                let lastModified = null;

                const source = new EventSource(window.location.origin + "/hot-reload?href=${href}");
                console.log('[HotReload] Started EventSource connection for hot reload');

                source.onmessage = (event) => {
                    const newLastModified = event.data;

                    if (lastModified && lastModified !== newLastModified) {
                        console.log('[HotReload] Detected changes, preparing reload...');
                        clearTimeout(timeoutId);

                        timeoutId = setTimeout(() => {
                            const scrollInfo = getScrollInfo();

                            globalThis.___hrcStates = preserveReactiveComponentsState();
                            console.log('[HotReload] Preserved states:', globalThis.___hrcStates);

                            document.documentElement.innerHTML = JSON.parse(event.data).html;

                            scrollInfo.forEach((item) => {
                                const hotReloadScroll = item?.hotReloadScroll;
                                const el = hotReloadScroll ? document.querySelector('[data-hot-reload-scroll="' + hotReloadScroll + '"]') : document.querySelector('body');

                                if (el) {
                                    el.scrollTo(item.scrollLeft, item.scrollTop);
                                    console.log('[HotReload] Restored scroll position for', hotReloadScroll || 'body');
                                }
                            });

                            recreateScripts(document.querySelector('head'));
                            recreateScripts(document.body);
                            console.log('[HotReload] DOM updated and scripts recreated');
                        }, 500);
                    }

                    lastModified = newLastModified;
                };

                function getScrollInfo() {
                    console.log('[HotReload] Starting get scroll info');
                    const body = document.querySelector('body');
                    const scrollInfo = [{ scrollLeft: body.scrollLeft, scrollTop: body.scrollTop }];
                    const elements = document.querySelectorAll('[data-hot-reload-scroll]');

                    for (const element of elements) {
                        if (element.scrollLeft || element.scrollTop) {
                            scrollInfo.push({
                                hotReloadScroll: element.dataset?.hotReloadScroll,
                                scrollLeft: element.scrollLeft,
                                scrollTop: element.scrollTop
                            });
                        }
                    }

                    console.log('[HotReload] Scroll info', scrollInfo);

                    return scrollInfo;
                }

                function recreateScripts(container) {
                    console.log('[HotReload] Recreating scripts for', container.tagName);
                    const scripts = container.querySelectorAll('script');

                    for (const script of scripts) {
                        if (script.dataset.hotReload) {
                            console.log('[HotReload] Skipping hot reload script');
                            continue;
                        }

                        if (script.src) {
                            console.log('[HotReload] Recreating external script:', script.src);
                            const newScript = document.createElement('script');

                            for (let i = 0; i < script.attributes.length; i++) {
                                const attribute = script.attributes[i];
                                const value = attribute.name === 'src' ? attribute.value + '?r=' + Math.random() : attribute.value;
                                newScript.setAttribute(attribute.name, value);
                            }

                            container.appendChild(newScript);
                        } else {
                            console.log('[HotReload] Recreating inline script');
                            const newInlineScript = document.createElement('script');

                            for (let i = 0; i < script.attributes.length; i++) {
                                const attribute = script.attributes[i];
                                newInlineScript.setAttribute(attribute.name, attribute.value);
                            }

                            newInlineScript.innerHTML = script.innerHTML;
                            container.appendChild(newInlineScript);
                        }
                    }
                    console.log('[HotReload] Finished recreating scripts for', container.tagName);
                }

                function preserveReactiveComponentsState() {
                    try {
                        const states = new Map();
                        console.log('[HotReload] Starting state preservation');

                        const elements = document.querySelectorAll('[data-hrc]');
                        console.log('[HotReload] Elements to preserve:', elements.length);

                        elements.forEach(element => {
                            console.log('[HotReload] Element state size:', element.state?.size);
                            if (element.state?.size) {
                                const stateObj = {};

                                element.state?.forEach((signal, key) => {
                                    stateObj[key] = signal();
                                });

                                states.set(element.dataset.hrc, stateObj);
                                console.log('[HotReload] Preserved state for component:', element.dataset.hrc, stateObj);
                            }
                        });

                        return states;
                    } catch (error) {
                        console.error('[HotReload] Error preserving state:', error);
                        return new Map();
                    }
                }

                function restoreReactiveComponentsState(element) {
                    if (!globalThis.___hrcStates?.size) {
                        console.log('[HotReload] No states to restore');
                        return;
                    }

                    if (element.state?.size) {
                        for (const [id, state] of globalThis.___hrcStates) {
                            if (id === element.dataset.hrc) {
                                console.log('[HotReload] Restoring state for component:', id, state);

                                Object.entries(state).forEach(([key, value]) => {
                                    element.setState(key, value);
                                });

                                globalThis.___hrcStates.delete(id);
                                console.log('[HotReload] Successfully restored state for component:', id);
                                break;
                            }
                        }
                    }
                }
            `}
        </script>
    ) : null;
}
