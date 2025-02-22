const CAMEL_PROPS =
    /^(?:accent|alignment|arabic|baseline|cap|clip(?!PathU)|color|dominant|fill|flood|font|glyph(?!R)|horiz|image(!S)|letter|lighting|marker(?!H|W|U)|overline|paint|pointer|shape|stop|strikethrough|stroke|text(?!L)|transform|underline|unicode|units|v|vector|vert|word|writing|x(?!C))[A-Z]/;
const CAMEL_REPLACE = /[A-Z0-9]/g;
const ON_ANI = /^on(Ani|Tra|Tou|BeforeInp|Compo)/;

function __jsxTemplate(string) {
    return string.replace(/>,\s*</g, "><");
}

function __jsxComponent(Component, props, children) {
    const finalProps = Array.isArray(props) ? props.reduce((acc, prop) => Object.assign(acc, prop), {}) : props;

    return Component({ ...finalProps, children });
}

function __jsxSpread(obj) {
    const result = [];
    for (const [propKey, propValue] of Object.entries(obj)) {
        if (propValue === null || propValue === undefined) continue;

        const normalizedKey = normalizeAttributeName(propKey);

        if (typeof propValue === "boolean") {
            if (propValue) {
                result.push(normalizedKey);
            }
            continue;
        }
        result.push(`${normalizedKey}="${propValue}"`);
    }
    return result.length ? ` ${result.join(" ")}` : "";
}

function normalizeAttributeName(name) {
    const lowerCased = name.toLowerCase();

    if (name === "className") return "class";
    if (name === "htmlFor") return "for";
    if (name === "acceptCharset") return "accept-charset";
    if (name === "httpEquiv") return "http-equiv";
    if (name === "imageRendering") return "image-rendering";

    if (lowerCased[0] === "o" && lowerCased[1] === "n") {
        if (lowerCased === "ondoubleclick") return "ondblclick";
        if (ON_ANI.test(name)) return lowerCased;
        return lowerCased;
    }

    if (name.startsWith("aria")) {
        return `aria-${name.slice(4).toLowerCase()}`;
    }

    if (CAMEL_PROPS.test(name)) {
        return name.replace(CAMEL_REPLACE, "-$&").toLowerCase();
    }

    return name;
}

globalThis.__jsxComponent = __jsxComponent;
globalThis.__jsxSpread = __jsxSpread;
globalThis.__jsxTemplate = __jsxTemplate;
