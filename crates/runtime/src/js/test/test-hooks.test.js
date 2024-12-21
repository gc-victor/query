import { beforeEach, afterEach, describe, expect, test, beforeAll, afterAll } from "query:test";

const sequence = [];

beforeAll(() => {
    sequence.push("beforeAllGlobal");
});

afterAll(() => {
    // This is not going to be printed as is executed before printing the testsResults
});

beforeEach(() => {
    sequence.push("beforeEachGlobal");
});

afterEach(() => {
    sequence.push("afterEachGlobal");
});

describe("Complex Hook Interactions", () => {
    beforeAll(() => {
        sequence.push("beforeAllSuite");
    });

    beforeEach(() => {
        sequence.push("beforeEach");
    });

    afterEach(() => {
        sequence.push("afterEach");
    });

    afterAll(() => {
        sequence.push("afterAllSuite");
    });

    test("should print initial befores", () => {
        // Here's why these hooks are printed:
        // 1. beforeAllGlobal: Runs once before all tests in the file
        // 2. beforeAllSuite: Runs once before all tests in this describe block
        // 3. beforeEachGlobal: Runs before each test from global scope
        // 4. beforeEach: Runs before each test in this describe block
        expect(sequence).toEqual(["beforeAllGlobal","beforeAllSuite","beforeEachGlobal","beforeEach"]);
    });

    test("should print initial befores and afters", () => {
        // Here's why these hooks are printed:
        // 1. beforeAllGlobal: First runs once before all tests in file
        // 2. beforeAllSuite: Runs once before all tests in this describe block
        // 3. beforeEachGlobal: Runs before each test from global scope
        // 4. beforeEach: Runs before each test in this describe block
        // 5. afterEach: Runs after each test in this describe block
        // 6. afterEachGlobal: Runs after the first test completes
        // 7. afterAllSuite: Runs when all tests in the suite finish
        // 8. beforeEachGlobal: Runs again before the second test from global scope
        // 9. beforeEach: Runs again before the second test in this describe block
        expect(sequence).toEqual([
            "beforeAllGlobal",
            "beforeAllSuite",
            "beforeEachGlobal",
            "beforeEach",
            "afterEach",
            "afterEachGlobal",
            "afterAllSuite",
            "beforeEachGlobal",
            "beforeEach",
        ]);
    });
});
