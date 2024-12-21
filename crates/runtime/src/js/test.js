class TestSuite {
    #filename = null;
    #testResults = [];
    #currentSuite = null;
    #fileHooks = {
        beforeEach: [],
        afterEach: [],
        beforeAll: [],
        afterAll: [],
    };
    #suiteHooks = {};

    constructor() {
        this.testNamePattern = null;
    }

    #now() {
        return Number(process.hrtime.bigint());
    }

    describe(name, fn) {
        this.#currentSuite = name;
        this.#suiteHooks[name] = {
            beforeEach: [],
            afterEach: [],
            beforeAll: [],
            afterAll: [],
        };

        try {
            fn();
        } catch (error) {
            print(`Error in test suite "${name}":`);
            print(error.message);
            print(error.stack);
        }
        this.#currentSuite = null;
    }

    beforeEach(fn) {
        if (this.#currentSuite) {
            this.#suiteHooks[this.#currentSuite].beforeEach.push(fn);
        } else {
            this.#fileHooks.beforeEach.push(fn);
        }
    }

    afterEach(fn) {
        if (this.#currentSuite) {
            this.#suiteHooks[this.#currentSuite].afterEach.push(fn);
        } else {
            this.#fileHooks.afterEach.push(fn);
        }
    }

    beforeAll(fn) {
        if (this.#currentSuite) {
            this.#suiteHooks[this.#currentSuite].beforeAll.push(fn);
        } else {
            this.#fileHooks.beforeAll.push(fn);
        }
    }

    afterAll(fn) {
        if (this.#currentSuite) {
            this.#suiteHooks[this.#currentSuite].afterAll.push(fn);
        } else {
            this.#fileHooks.afterAll.push(fn);
        }
    }

    async test(name, fn) {
        const start = this.#now();
        const filename = this.#filename;
        const currentSuite = this.#currentSuite;

        try {
            for (const beforeFn of this.#fileHooks.beforeAll) {
                beforeFn();
            }

            this.#fileHooks.beforeAll = [];

            if (currentSuite) {
                for (const beforeFn of this.#suiteHooks[currentSuite].beforeAll) {
                    beforeFn();
                }

                this.#suiteHooks[currentSuite].beforeAll = [];
            }

            for (const beforeFn of this.#fileHooks.beforeEach) {
                beforeFn();
            }

            if (currentSuite) {
                for (const beforeFn of this.#suiteHooks[currentSuite].beforeEach) {
                    beforeFn();
                }
            }

            if (fn.constructor.name === "AsyncFunction") {
                await fn();
            } else {
                fn();
            }

            this.afterHooks(currentSuite);

            this.#testResults.push({
                filename: filename,
                currentSuite: currentSuite || "Global",
                name,
                start: start,
                end: this.#now(),
            });
        } catch (error) {
            this.afterHooks(currentSuite);

            this.#testResults.push({
                filename: filename,
                currentSuite: currentSuite || "Global",
                name,
                start: start,
                end: this.#now(),
                error: error.message,
            });
        }
    }

    afterHooks(currentSuite) {
        if (currentSuite) {
            for (const afterFn of this.#suiteHooks[currentSuite].afterEach) {
                afterFn();
            }
        }

        for (const afterFn of this.#fileHooks.afterEach) {
            afterFn();
        }

        this.#suiteHooks[currentSuite].afterEach = [];

        if (currentSuite) {
            for (const afterFn of this.#suiteHooks[currentSuite].afterAll) {
                afterFn();
            }
        }
    }

    expect(actual) {
        return {
            toBe(expected) {
                if (actual !== expected) {
                    throw new Error(`Expected ${JSON.stringify(actual)} to be ${JSON.stringify(expected)}`);
                }
            },

            toEqual(expected) {
                const actualStr = JSON.stringify(actual);
                const expectedStr = JSON.stringify(expected);
                if (actualStr !== expectedStr) {
                    throw new Error(`Expected ${actualStr} to equal ${expectedStr}`);
                }
            },

            toDeepEqual(expected) {
                const checkDeep = (a, b) => {
                    if (a === b) return true;
                    if (typeof a !== "object" || typeof b !== "object") return false;
                    if (a === null || b === null) return false;

                    const keysA = Object.keys(a);
                    const keysB = Object.keys(b);

                    if (keysA.length !== keysB.length) return false;

                    for (const key of keysA) {
                        if (!keysB.includes(key)) return false;
                        if (!checkDeep(a[key], b[key])) return false;
                    }

                    return true;
                };

                if (!checkDeep(actual, expected)) {
                    throw new Error(`Expected ${JSON.stringify(actual)} to deeply equal ${JSON.stringify(expected)}`);
                }
            },

            toBeTruthy() {
                if (!actual) {
                    throw new Error(`Expected ${actual} to be truthy`);
                }
            },

            toBeFalsy() {
                if (actual) {
                    throw new Error(`Expected ${actual} to be falsy`);
                }
            },

            toContain(item) {
                if (!actual.includes(item)) {
                    throw new Error(`Expected ${JSON.stringify(actual)} to contain ${JSON.stringify(item)}`);
                }
            },

            toThrow() {
                if (typeof actual !== "function") {
                    throw new Error("Expected a function to test for thrown errors");
                }
                try {
                    actual();
                    throw new Error("Expected function to throw an error");
                } catch (e) {
                    // Successfully caught error
                    return true;
                }
            },
        };
    }

    spyOn(obj, method, returnValue) {
        const stats = {
            callCount: 0,
            called: false,
            calls: [],
            returnValue: null,
        };

        Object.defineProperty(obj, method, {
            value: (...args) => {
                stats.callCount++;
                stats.called = true;
                stats.calls.push(...args);
                stats.returnValue = returnValue.apply(null, args);
                return stats.returnValue;
            },
            writable: true,
        });

        return stats;
    }

    async testsResults() {
        for (const afterFn of this.#fileHooks.afterAll) {
            afterFn();
        }

        return this.#testResults;
    }

    /**
     * @param {string} filename
     */
    setFilename(filename) {
        this.#filename = filename;
    }
}

const testSuite = new TestSuite();

export const beforeAll = testSuite.beforeAll.bind(testSuite);
export const beforeEach = testSuite.beforeEach.bind(testSuite);
export const afterAll = testSuite.afterAll.bind(testSuite);
export const afterEach = testSuite.afterEach.bind(testSuite);
export const describe = testSuite.describe.bind(testSuite);
export const test = testSuite.test.bind(testSuite);
export const expect = testSuite.expect.bind(testSuite);
export const spyOn = testSuite.spyOn.bind(testSuite);

globalThis.___testsResults = testSuite.testsResults.bind(testSuite);
globalThis.___testFilename = testSuite.setFilename.bind(testSuite);
