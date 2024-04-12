const ___formData = Symbol();

// @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData
class FormData {
    constructor(form) {
        this[___formData] = [];

        if (form !== undefined) {
            throw new TypeError("Failed to construct 'FormData': parameters are not allowed.");
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/append
    append(key, initValue /* value */, filename) {
        const value = (initValue instanceof Blob || initValue instanceof ArrayBuffer) && filename ? new File([initValue], filename, { type: initValue.type }) : initValue;

        if (this.has(key)) {
            this[___formData] = this[___formData].map((pair) => {
                if (pair[0] === key) {
                    const oldValue = pair[1];
                    const newValue = stringifyValue(value);

                    if (Array.isArray(oldValue)) {
                        oldValue.push(newValue);
                        return [key, oldValue];
                    }

                    return [key, [oldValue, newValue]];
                }

                return pair;
            });
        } else {
            this[___formData].push([key, stringifyValue(value)]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/get
    get(key) {
        const result = this[___formData].find((pair) => pair[0] === key);

        if (result === undefined) {
            return null;
        }

        const value = result[1];

        if (value === undefined) {
            return null;
        }

        return Array.isArray(value) ? value[0] : value;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/getAll
    getAll(key) {
        const result = this[___formData].find((pair) => pair[0] === key);

        if (result === undefined) {
            return [];
        }

        return result[1];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/has
    has(key) {
        return this[___formData].some((pair) => pair[0] === key);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/set
    set(key, value) {
        if (this.has(key)) {
            this[___formData] = this[___formData].map((pair) => (pair[0] === key ? [key, stringifyValue(value)] : pair));
        } else {
            this[___formData].push([key, stringifyValue(value)]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/delete
    delete(key) {
        this[___formData] = this[___formData].filter((pair) => pair[0] !== key);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/entries
    entries() {
        return this[___formData][Symbol.iterator]();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/keys
    keys() {
        return this[___formData].map((pair) => pair[0])[Symbol.iterator]();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/values
    values() {
        return this[___formData].map((pair) => pair[1])[Symbol.iterator]();
    }

    toString() {
        return "[object FormData]";
    }

    get [Symbol.toStringTag]() {
        return "FormData";
    }
}

globalThis.FormData = FormData;

function stringifyValue(value) {
    return typeof value === "string" || value instanceof Blob ? value : String(value);
}
