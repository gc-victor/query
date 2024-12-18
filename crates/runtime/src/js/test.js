class TestSuite {
    #filename = null;
    #testResults = [];
    #currentSuite = null;
    #beforeEachFns = [];

    constructor() {
        this.testNamePattern = null;

        this.colors = {
            green: "\x1b[32m",
            red: "\x1b[31m",
            reset: "\x1b[0m",
            cyan: "\x1b[36m",
            grey: "\x1b[90m",
            bold: "\x1b[1m",
        };
    }

    #now() {
        return Number(process.hrtime.bigint());
    }

    describe(name, fn) {
        this.#currentSuite = name;
        this.#beforeEachFns = [];
        try {
            fn();
        } catch (error) {
            print(`${this.colors.red}Error in test suite "${name}":${this.colors.reset}`, error);
        }
        this.#currentSuite = null;
    }

    beforeEach(fn) {
        this.#beforeEachFns.push(fn);
    }

    async test(name, fn) {
        if (this.testNamePattern && !name.includes(this.testNamePattern)) {
            return;
        }

        const start = this.#now();
        const filename = this.#filename;
        const currentSuite = this.#currentSuite;

        try {
            for (const beforeFn of this.#beforeEachFns) {
                beforeFn();
            }

            if (fn.constructor.name === "AsyncFunction") {
                await fn();
            } else {
                fn();
            }

            this.#testResults.push({
                filename: filename,
                currentSuite: currentSuite,
                name,
                start: start,
                end: this.#now(),
            });
        } catch (error) {
            this.#testResults.push({
                filename: filename,
                currentSuite: currentSuite,
                name,
                start: start,
                end: this.#now(),
                error: error.message,
            });
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

    testsResults() {
        return this.#testResults;
    }

    printGlobalTestSummary() {
        let totalTests = 0;
        let totalPassed = 0;
        let totalFailed = 0;
        const totalResults = [];

        print(`${this.#testResults.length}`);
        console.log(this.#testResults);

        for (const results of this.#testResults) {
            const filename = results.filename;
            const fileTotal = results.length;
            const filePassed = results.filter((t) => t.passed).length;
            const fileFailed = fileTotal - filePassed;

            totalTests += fileTotal;
            totalPassed += filePassed;
            totalFailed += fileFailed;

            totalResults.push({
                filename,
                total: fileTotal,
                passed: filePassed,
                failed: fileFailed,
                results,
            });
        }

        globalThis.___totalFailed = totalFailed;

        print(`\nFiles: ${totalResults.length}`);
        print(`Tests: ${totalTests}`);
        print(`${this.colors.green}Passed: ${totalPassed}${this.colors.reset}`);
        print(`${this.colors.red}Failed: ${totalFailed}${this.colors.reset}`);

        if (totalResults[0].results.length) {
            const firstTest = totalResults[0].results[0];
            const lastTest = totalResults.at(-1).results.at(-1);
            const totalTime = (lastTest.end - firstTest.start).toFixed(2);
            print(`${this.colors.grey}Time: ${totalTime}ms${this.colors.reset}`);

            for (const file of totalResults) {
                if (file.failed === 0) {
                    continue;
                }

                print(`\n${this.colors.red}File: ${file.filename}`);
                print(`Failed: ${file.failed} tests${this.colors.reset}`);

                const failedTests = file.results
                    .filter((t) => !t.passed)
                    .map((t) => ({
                        name: t.name,
                        message: t.error.message.split("\n").map((l) => `- ${l}`),
                    }));

                for (const test of failedTests) {
                    print(`${this.colors.red}${test.name}\n${this.colors.red}${test.message.join("\n  ")}${this.colors.reset}`);
                }
            }
        }
    }

    /**
     * @param {string} filename
     */
    setFilename(filename) {
        this.#filename = filename;
    }
}

const testSuite = new TestSuite();

export const describe = testSuite.describe.bind(testSuite);
export const beforeEach = testSuite.beforeEach.bind(testSuite);
export const test = testSuite.test.bind(testSuite);
export const expect = testSuite.expect.bind(testSuite);
export const spyOn = testSuite.spyOn.bind(testSuite);

globalThis.___testsResults = globalThis.___testsResults || testSuite.testsResults.bind(testSuite);
globalThis.___testFilename = globalThis.___testFilename || testSuite.setFilename.bind(testSuite);
