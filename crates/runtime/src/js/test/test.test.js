import { beforeEach, describe, expect, test } from 'query:test';

// Test basic test functionality
describe('Basic Test Functionality', () => {
    test('should pass when assertion is true', () => {
        expect(true).toBeTruthy();
    });

    test('should pass when values are equal', () => {
        expect(1 + 1).toBe(2);
    });
});

// Test beforeEach functionality
describe('beforeEach Functionality', () => {
    let counter;

    beforeEach(() => {
        counter = 0;
    });

    test('counter should start at 0', () => {
        expect(counter).toBe(0);
    });

    test('counter should be reset for each test', () => {
        counter++;
        expect(counter).toBe(1);
    });

    test('counter should be reset again', () => {
        expect(counter).toBe(0);
    });
});

// Test expect matchers
describe('Expect Matchers', () => {
    test('toBe matcher', () => {
        expect(42).toBe(42);
        expect('hello').toBe('hello');
        expect(true).toBe(true);
    });

    test('toEqual matcher', () => {
        const obj = { a: 1, b: 2 };
        const arr = [1, 2, 3];
        expect(obj).toEqual({ a: 1, b: 2 });
        expect(arr).toEqual([1, 2, 3]);
    });
    
    test('toDeepEqual matcher', () => {
        const obj1 = { 
            a: 1, 
            b: { 
                c: 2, 
                d: [1, 2, 3]
            }
        };
        const obj2 = { 
            a: 1, 
            b: { 
                c: 2, 
                d: [1, 2, 3]
            }
        };
        expect(obj1).toDeepEqual(obj2);
        expect([{x: 1}, {y: 2}]).toDeepEqual([{x: 1}, {y: 2}]);
    });


    test('toBeTruthy matcher', () => {
        expect(true).toBeTruthy();
        expect(1).toBeTruthy();
        expect('hello').toBeTruthy();
    });

    test('toBeFalsy matcher', () => {
        expect(false).toBeFalsy();
        expect(0).toBeFalsy();
        expect('').toBeFalsy();
        expect(null).toBeFalsy();
        expect(undefined).toBeFalsy();
    });

    test('toContain matcher', () => {
        expect([1, 2, 3]).toContain(1);
        expect('hello world').toContain('world');
    });

    test('toThrow matcher', () => {
        const throwingFn = () => {
            throw new Error('test error');
        };
        expect(throwingFn).toThrow();
    });
});

// Test async functionality and promises
describe('Async and Promise Tests', () => {
    test('should handle async functions and promises', async () => {
        const asyncFn = async () => {
            return Promise.resolve('promise resolved');
        };

        const result = await asyncFn();
        expect(result).toBe('promise resolved');
    });

    test('should handle promise rejection', async () => {
        const asyncFn = async () => {
            return new Promise((_, reject) => {
                setTimeout(() => reject(new Error('promise rejected')), 0);
            });
        };

        let caught = false;
        try {
            await asyncFn();
        } catch (e) {
            caught = true;
            expect(e.message).toBe('promise rejected');
        }
        expect(caught).toBeTruthy();
    });
});

// Test error conditions
describe('Error Handling', () => {
    test('should catch failed toBe assertions', () => {
        let caught = false;
        try {
            expect(1).toBe(2);
        } catch (e) {
            caught = true;
            expect(e.message).toContain('Expected 1 to be 2');
        }
        expect(caught).toBeTruthy();
    });

    test('should catch failed toEqual assertions', () => {
        let caught = false;
        try {
            expect({ a: 1 }).toEqual({ a: 2 });
        } catch (e) {
            caught = true;
            expect(e.message).toContain('Expected');
        }
        expect(caught).toBeTruthy();
    });

    test('should catch failed toContain assertions', () => {
        let caught = false;
        try {
            expect([1, 2, 3]).toContain(4);
        } catch (e) {
            caught = true;
            expect(e.message).toContain('Expected');
        }
        expect(caught).toBeTruthy();
    });
});
