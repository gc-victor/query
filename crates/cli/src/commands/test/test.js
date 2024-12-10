import { timer } from './timer';

// Store test suites and their tests
let currentSuite = null;
let beforeEachFns = [];
const testResults = [];

// Colors for console output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    reset: '\x1b[0m',
    cyan: '\x1b[36m',
    grey: '\x1b[90m',
    bold: "\x1b[1m"
};

export function describe(name, fn) {
    currentSuite = name;
    beforeEachFns = [];
    try {
        fn();
    } catch (error) {
        print(`${colors.red}Error in test suite "${name}":${colors.reset}`, error);
    }
    currentSuite = null;
}

export function beforeEach(fn) {
    beforeEachFns.push(fn);
}

export function test(name, fn) {
    if (___testNamePattern && !name.includes(___testNamePattern)) {
        return;
    }
    
    const t = timer.start();
    const testName = `${currentSuite ? `${currentSuite} > ` : ""}${colors.bold}${name}${colors.reset}`;
    try {
        for (const beforeFn of beforeEachFns) {
            beforeFn();
        }

        fn();
        print(`${colors.green}✓ ${colors.reset}${testName} ${colors.grey}[${t.end()}]${colors.reset}`);
        testResults.push({ name: testName, passed: true, start: t.startTime, end: t.endTime });
    } catch (error) {
        print(`${colors.red}✗ ${colors.reset}${testName} ${colors.grey}[${t.end()}]${colors.reset}\n ${colors.red} - ${error.message}${colors.reset}`);
        testResults.push({ name: testName, passed: false, start: t.startTime, end: t.endTime, error });
    }
}

export function expect(actual) {
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
                if (typeof a !== 'object' || typeof b !== 'object') return false;
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
            if (typeof actual !== 'function') {
                throw new Error('Expected a function to test for thrown errors');
            }
            try {
                actual();
                throw new Error('Expected function to throw an error');
            } catch (e) {
                // Successfully caught error
                return true;
            }
        }
    };
}

function printGlobalTestSummary() {
    let totalTests = 0;
    let totalPassed = 0;
    let totalFailed = 0;
    const totalResults = [];

    // Process results from each test file
    for (const [fileName, results] of Object.entries(globalThis.___testResults)) {
        const fileTotal = results.length;
        const filePassed = results.filter(t => t.passed).length;
        const fileFailed = fileTotal - filePassed;

        totalTests += fileTotal;
        totalPassed += filePassed;
        totalFailed += fileFailed;

        totalResults.push({
            fileName,
            total: fileTotal,
            passed: filePassed,
            failed: fileFailed,
            results
        });
    }
    
    globalThis.___totalFailed = totalFailed;

    print(`\nFiles: ${totalResults.length}`);
    print(`Tests: ${totalTests}`);
    print(`${colors.green}Passed: ${totalPassed}${colors.reset}`);
    print(`${colors.red}Failed: ${totalFailed}${colors.reset}`);
    
    const firstTest = totalResults[0].results[0];
    const lastTest = totalResults.at(-1).results.at(-1);    
    const totalTime = (lastTest.end - firstTest.start).toFixed(2);
    print(`${colors.grey}Time: ${totalTime}ms${colors.reset}`);

    for (const file of totalResults) {
        if (file.failed === 0) {
            continue;
        }

        print(`\n${colors.red}File: ${file.fileName}`);
        print(`Failed: ${file.failed} tests${colors.reset}`);

        const failedTests = file.results.filter(t => !t.passed).map(t => ({
            name: t.name,
            message: t.error.message.split('\n').map(l => `- ${l}`)
        }));

        for (const test of failedTests) {
            print(`${colors.red}${test.name}\n${colors.red}${test.message.join('\n  ')}${colors.reset}`);
        }
    }
}

globalThis.describe = describe;
globalThis.beforeEach = beforeEach;
globalThis.test = test;
globalThis.expect = expect;

globalThis.___testResults = globalThis.___testResults || {};
globalThis.___printGlobalTestSummary = printGlobalTestSummary;
globalThis.___printTestSummary = (fileName) => globalThis.___testResults[fileName] = testResults;
