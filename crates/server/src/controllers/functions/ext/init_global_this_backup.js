// CREDIT: https://github.com/rscarson/rustyscript/blob/0fd168e3b82106c3c4551f1e744b113e26f2b109/src/ext/rustyscript/rustyscript.js#L52
globalThis.init_global_this_backup = () => {
    globalThis.global_this_backup = cloneJSON(globalThis);
}

globalThis.reset_global_this_backup = () => {
    let backup = cloneJSON(globalThis.global_this_backup);
    for (const key of Object.keys(globalThis)) {
        if (backup[key]) continue;
        globalThis[key] = undefined;
    }

    for (const key of Object.keys(backup)) {
        globalThis[key] = backup[key];
    }
}

// structuredClone not available in this context
// CREDIT: https://github.com/rhysd/fast-json-clone/blob/0da6753335c52bc28eaebc89a3174305a31979bf/index.ts#L22C1-L35C2
function cloneJSON(value, cache = new WeakMap()) {
    if (typeof value !== 'object' || value === null) {
        return value;
    } else if (cache.has(value)) {
        return cache.get(value);
    } else if (Array.isArray(value)) {
        const result = value.map(e => (typeof e !== 'object' || e === null ? e : cloneJSON(e, cache)));
        cache.set(value, result);
        return result;
    } else {
        const ret = {};
        cache.set(value, ret);
        for (const k in value) {
            const v = value[k];
            ret[k] = typeof v !== 'object' || v === null ? v : cloneJSON(v, cache);
        }
        return ret;
    }
}
