import { IS_DEVELOPMENT } from "@/config/server/server.constants";

// https://marketplace.visualstudio.com/items?itemName=Tobermory.es6-string-html
const js = String.raw;

export function HotReload({ href }: { href: string }) {
    return IS_DEVELOPMENT ? (
        <script
            data-hot-reload="true"
            // biome-ignore lint/security/noDangerouslySetInnerHtml: <explanation>
            dangerouslySetInnerHTML={{
                __html: js`
    let lastModified = null;

    const source = new EventSource(window.location.origin + "/hot-reload?href=${href}");

    source.onmessage = (event) => {
        const newLastModified = event.data;

        if (lastModified && lastModified !== newLastModified) {
            const scrollInfo = getScrollInfo();

            document.documentElement.innerHTML = JSON.parse(event.data).html;

            recreateScripts(document.querySelector('head'));
            recreateScripts(document.body);

            scrollInfo.forEach((item) => {
                const hotReloadScroll = item?.hotReloadScroll;
                const el = hotReloadScroll ? document.querySelector('[data-hot-reload-scroll="' + hotReloadScroll + '"]') : document.querySelector('body');

                if (el) {
                    el.scrollTo(item.scrollLeft, item.scrollTop);
                }
            });
        }

        lastModified = newLastModified;
    };

    function getScrollInfo() {
        const body = document.querySelector('body');
        const scrollInfo = Array.from(document.querySelectorAll('[data-hot-reload-scroll]')).reduce((acc, element) => {
            if (element.scrollLeft || element.scrollTop) {
                return acc.concat({ hotReloadScroll: element.dataset?.hotReloadScroll, scrollLeft: element.scrollLeft, scrollTop: element.scrollTop });
            }

            return acc;
        }, [{ scrollLeft: body.scrollLeft, scrollTop: body.scrollTop }]);

        return scrollInfo;
    }

    function recreateScripts(container) {
        const scripts = container.querySelectorAll('script');

        for (const script of scripts) {
            if (script.dataset.hotReload) {
                continue;
            }

            if (script.src) {
                const newScript = document.createElement('script');

                for (let i = 0; i < script.attributes.length; i++) {
                    const attribute = script.attributes[i];
                    const value = attribute.name === 'src' ? attribute.value + '?r=' + Math.random() : attribute.value;
                    newScript.setAttribute(attribute.name, value);
                }

                container.appendChild(newScript);
            } else {
                const newInlineScript = document.createElement('script');

                for (let i = 0; i < script.attributes.length; i++) {
                    const attribute = script.attributes[i];
                    newInlineScript.setAttribute(attribute.name, attribute.value);
                }

                newInlineScript.innerHTML = script.innerHTML;
                container.appendChild(newInlineScript);
            }
        }
    }`,
            }}
        />
    ) : null;
}
