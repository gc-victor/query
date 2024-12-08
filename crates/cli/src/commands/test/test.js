// Store test suites and their tests
let currentSuite = null;
let beforeEachFns = [];
const testResults = [];

// Colors for console output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    reset: '\x1b[0m',
    cyan: '\x1b[36m'
};

// Main describe function for test suites
export function describe(name, fn) {
    print(`${colors.cyan}Test Suite: ${name}${colors.reset}`);
    currentSuite = name;
    beforeEachFns = [];
    try {
        fn();
    } catch (error) {
        console.error(`${colors.red}Error in test suite "${name}":${colors.reset}`, error);
    }
    currentSuite = null;
}

// beforeEach function for setup
export function beforeEach(fn) {
    beforeEachFns.push(fn);
}

// test function for individual tests
export function test(name, fn) {
    const testName = `${currentSuite} - ${name}`;
    try {
        // Run beforeEach functions
        for (const beforeFn of beforeEachFns) {
            beforeFn();
        }

        fn();
        print(`${colors.green}✓ ${testName}${colors.reset}`);
        testResults.push({ name: testName, passed: true });
    } catch (error) {
        print(`${colors.red}✗ ${testName}\n  ${error.message}${colors.reset}`);
        testResults.push({ name: testName, passed: false, error });
    }
}

// expect function for assertions
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

// Store test results globally
globalThis.__testResults = globalThis.__testResults || {};

// Function to print test summary
function printTestSummary(fileName) {
    globalThis.__testResults[fileName] = testResults;

    const total = testResults.length;
    const passed = testResults.filter(t => t.passed).length;
    const failed = total - passed;

    print(`\nTotal Tests: ${total}`);
    print(`${colors.green}Passed: ${passed}${colors.reset}`);
    print(`${colors.red}Failed: ${failed}${colors.reset}`);
}

// Function to print test summary globally
function printGlobalTestSummary() {
    let totalTests = 0;
    let totalPassed = 0;
    let totalFailed = 0;
    const fileResults = [];

    // Process results from each test file
    for (const [fileName, results] of Object.entries(globalThis.__testResults)) {
        const fileTotal = results.length;
        const filePassed = results.filter(t => t.passed).length;
        const fileFailed = fileTotal - filePassed;

        totalTests += fileTotal;
        totalPassed += filePassed;
        totalFailed += fileFailed;

        fileResults.push({
            fileName,
            total: fileTotal,
            passed: filePassed,
            failed: fileFailed,
            results
        });
    }

    print('\nTotal Tests Summary:');
    print(`- Total Files: ${fileResults.length}`);
    print(`- Total Tests: ${totalTests}`);
    print(`${colors.green}- Total Passed: ${totalPassed}${colors.reset}`);
    print(`${colors.red}- Total Failed: ${totalFailed}${colors.reset}`);

    for (const file of fileResults) {
        if (file.failed === 0) {
            continue;
        }

        print(`${colors.red}\nFile: ${file.fileName}${colors.reset}`);
        print(`${colors.red}Failed: ${file.failed} tests${colors.reset}`);

        const failedTests = file.results.filter(t => !t.passed).map(t => ({
            name: t.name,
            message: t.error.message.split('\n').map(l => `- ${l}`)
        }));

        for (const test of failedTests) {
            print(`${colors.red}${test.name}\n${test.message.join('\n  ')}${colors.reset}`);
        }
    }
}

// Make printGlobalTestSummary global
globalThis.__printGlobalTestSummary = printGlobalTestSummary;

// Make functions global
globalThis.describe = describe;
globalThis.beforeEach = beforeEach;
globalThis.test = test;
globalThis.expect = expect;
globalThis.__printTestSummary = printTestSummary;
