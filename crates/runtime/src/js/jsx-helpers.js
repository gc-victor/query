function __jsxTemplate(string) {
    return string.replace(/>,\s*</g, "><");
}

function __jsxComponent(Component, props, children) {
    const finalProps = Array.isArray(props) ? props.reduce((acc, prop) => Object.assign(acc, prop), {}) : props;
    return Component({ children, ...finalProps });
}

function __jsxSpread(obj) {
    const result = [];
    for (const [propKey, propValue] of Object.entries(obj)) {
        if (propValue === null || propValue === undefined) continue;

        if (typeof propValue === "boolean") {
            if (propValue) {
                result.push(propKey);
            }
            continue;
        }
        result.push(`${propKey}="${propValue}"`);
    }
    return result.join(" ");
}

globalThis.__jsxComponent = __jsxComponent;
globalThis.__jsxSpread = __jsxSpread;
globalThis.__jsxTemplate = __jsxTemplate;
