globalThis.console = {
    assert: (condition, ...args) => {
        if (!condition) {
            ___print.error("Assertion failed:", mapArgsAndJoin(args));
        }
    },
    debug: (...args) => ___print.debug(...mapArgsAndJoin(args)),
    error: (...args) => ___print.error(...mapArgsAndJoin(args)),
    info: (...args) => ___print.info(...mapArgsAndJoin(args)),
    log: (...args) => ___print.log(...mapArgsAndJoin(args)),
    warn: (...args) => ___print.warn(...mapArgsAndJoin(args)),
};

function mapArgsAndJoin(args) {
    if (args.length === 0) {
        return "";
    }

    if (args.length > 1 && typeof args[0] === "string") {
        const result = substitutions(args);

        if (result !== args[0]) {
            return [result];
        }
    }

    return args.map((arg) => {
        if (arg === null) {
            return "null";
        }

        if (arg instanceof Error) {
            return `${arg.message}\\n${arg.stack || ""}`;
        }

        if (typeof arg === "function") {
            return arg.toString();
        }

        if (typeof arg === "object" && !(arg instanceof Promise)) {
            return customStringify(arg);
        }

        return arg;
    });
}

function customStringify(obj, visited = new Set()) {
    if (obj === null || typeof obj !== "object") {
        return JSON.stringify(obj);
    }

    if (visited.has(obj)) {
        return '"[Circular]"';
    }

    visited.add(obj);

    const result = Array.isArray(obj)
        ? `[${obj.map((element) => customStringify(element, new Set(visited))).join(",")}]`
        : `{${Object.keys(obj)
              .map((key) => `"${key}":${customStringify(obj[key], new Set(visited))}`)
              .join(",")}}`;

    return result;
}

function substitutions(args) {
    let i = 0;

    const msg = args[0];
    const remainingArgs = args.slice(1);

    return msg.replace(/%[sdifoOc%]/g, (match) => {
        const arg = remainingArgs[i++];

        if (arg === undefined) return match;

        const formatMap = {
            "%%": () => "%",
            "%c": () => "",
            "%d": (arg) => Number.parseInt(arg, 10).toString(),
            "%f": (arg) => Number.parseFloat(arg).toString(),
            "%i": (arg) => Number.parseInt(arg, 10).toString(),
            "%o": JSON.stringify,
            "%O": JSON.stringify,
            "%s": String,
        };

        return formatMap[match] ? formatMap[match](arg) : match;
    });
}
