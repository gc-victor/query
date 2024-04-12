const ___file = Symbol();

class File extends Blob {
    constructor(fileBits, fileName, options) {
        super(fileBits, { type: options?.type || "", endings: options?.endings || "transparent" });

        this[___file] = {};
        this[___file].lastModified = options?.lastModified || Date.now();
        this[___file].name = fileName;
    }

    get lastModified() {
        return this[___file].lastModified;
    }

    get name() {
        return this[___file].name;
    }

    get webkitRelativePath() {
        return "";
    }

    get [Symbol.toStringTag]() {
        return "File";
    }
}

globalThis.File = File;
