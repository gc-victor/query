import { describe, expect, test } from "query:test";

test("should pass when assertion is true", () => {
    expect(true).toBeTruthy();
});

test("should pass when values are equal", () => {
    expect(1 + 1).toBe(2);
});

describe("Basic Test Functionality", () => {
    test("should pass when assertion is true", () => {
        expect(true).toBeTruthy();
    });

    test("should pass when values are equal", () => {
        expect(1 + 1).toBe(2);
    });
});

describe("Expect Matchers", () => {
    test("toBe matcher", () => {
        expect(42).toBe(42);
        expect("hello").toBe("hello");
        expect(true).toBe(true);
    });

    test("toEqual matcher", () => {
        const obj = { a: 1, b: 2 };
        const arr = [1, 2, 3];
        expect(obj).toEqual({ a: 1, b: 2 });
        expect(arr).toEqual([1, 2, 3]);
    });

    test("toDeepEqual matcher", () => {
        const obj1 = {
            a: 1,
            b: {
                c: 2,
                d: [1, 2, 3],
            },
        };
        const obj2 = {
            a: 1,
            b: {
                c: 2,
                d: [1, 2, 3],
            },
        };
        expect(obj1).toDeepEqual(obj2);
        expect([{ x: 1 }, { y: 2 }]).toDeepEqual([{ x: 1 }, { y: 2 }]);
    });

    test("toBeTruthy matcher", () => {
        expect(true).toBeTruthy();
        expect(1).toBeTruthy();
        expect("hello").toBeTruthy();
    });

    test("toBeFalsy matcher", () => {
        expect(false).toBeFalsy();
        expect(0).toBeFalsy();
        expect("").toBeFalsy();
        expect(null).toBeFalsy();
        expect(undefined).toBeFalsy();
    });

    test("toContain matcher", () => {
        expect([1, 2, 3]).toContain(1);
        expect("hello world").toContain("world");
    });

    test("should catch failed toMatch assertions", () => {
        let caught = false;
        try {
            expect("hello world").toMatch("xyz");
        } catch (e) {
            caught = true;
            expect(e.message).toContain("Expected");
        }
        expect(caught).toBeTruthy();
    });

    test("toThrow matcher", () => {
        const throwingFn = () => {
            throw new Error("test error");
        };
        expect(throwingFn).toThrow();
    });
});

describe("Async and Promise Tests", () => {
    test("should handle async functions and promises", async () => {
        const asyncFn = async () => {
            return Promise.resolve("promise resolved");
        };

        const result = await asyncFn();
        expect(result).toBe("promise resolved");
    });

    test("should handle promise rejection", async () => {
        const asyncFn = async () => {
            return new Promise((_, reject) => {
                reject(new Error("promise rejected"));
            });
        };

        let caught = false;
        try {
            await asyncFn();
        } catch (e) {
            caught = true;
            expect(e.message).toBe("promise rejected");
        }
        expect(caught).toBeTruthy();
    });
});

describe("Error Handling", () => {
    test("should catch failed toBe assertions", () => {
        let caught = false;
        try {
            expect(1).toBe(2);
        } catch (e) {
            caught = true;
            expect(e.message).toContain("Expected 1 to be 2");
        }
        expect(caught).toBeTruthy();
    });

    test("should catch failed toEqual assertions", () => {
        let caught = false;
        try {
            expect({ a: 1 }).toEqual({ a: 2 });
        } catch (e) {
            caught = true;
            expect(e.message).toContain("Expected");
        }
        expect(caught).toBeTruthy();
    });

    test("should catch failed toContain assertions", () => {
        let caught = false;
        try {
            expect([1, 2, 3]).toContain(4);
        } catch (e) {
            caught = true;
            expect(e.message).toContain("Expected");
        }
        expect(caught).toBeTruthy();
    });
});

describe("Negation", () => {
    test("should not match different values with toBe", () => {
        expect(42).not.toBe(43);
        expect("hello").not.toBe("world");
        expect(true).not.toBe(false);
    });

    test("should not match different objects with toEqual", () => {
        const obj = { a: 1, b: 2 };
        const arr = [1, 2, 3];
        expect(obj).not.toEqual({ a: 2, b: 3 });
        expect(arr).not.toEqual([4, 5, 6]);
    });

    test("should not deeply equal different nested objects", () => {
        const obj1 = {
            a: 1,
            b: {
                c: 2,
                d: [1, 2, 3],
            },
        };
        const obj2 = {
            a: 2,
            b: {
                c: 3,
                d: [4, 5, 6],
            },
        };
        expect(obj1).not.toDeepEqual(obj2);
        expect([{ x: 1 }, { y: 2 }]).not.toDeepEqual([{ x: 2 }, { y: 3 }]);
    });

    test("should not be truthy for falsy values", () => {
        expect(false).not.toBeTruthy();
        expect(0).not.toBeTruthy();
        expect("").not.toBeTruthy();
    });

    test("should not be falsy for truthy values", () => {
        expect(true).not.toBeFalsy();
        expect(1).not.toBeFalsy();
        expect("hello").not.toBeFalsy();
        expect({}).not.toBeFalsy();
        expect([]).not.toBeFalsy();
    });

    test("should not contain non-existent elements", () => {
        expect([1, 2, 3]).not.toContain(4);
        expect("hello world").not.toContain("xyz");
    });

    test("should not match invalid patterns", () => {
        expect("hello world").not.toMatch("xyz");
        expect("12345").not.toMatch(/[a-z]/);
    });

    test("should not throw for non-throwing functions", () => {
        const nonThrowingFn = () => {
            return "no error";
        };
        expect(nonThrowingFn).not.toThrow();
    });
});
