import { describe, beforeAll, beforeEach, afterEach, afterAll, test, expect } from "query:test";

beforeAll(() => print("File beforeAll"));
beforeEach(() => print("File beforeEach"));
afterEach(() => print("File afterEach"));
afterAll(() => print("File afterAll"));

describe("test suite", () => {
    beforeAll(() => print("Suite beforeAll"));
    beforeEach(() => print("Suite beforeEach"));
    afterEach(() => print("Suite afterEach"));
    afterAll(() => print("Suite afterAll"));

    test("example test", () => {
        print("Test execution");
        expect(true).toBeTruthy();
    });
});
