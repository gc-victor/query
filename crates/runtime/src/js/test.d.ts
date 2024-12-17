export type TestFunction = () => void;
export type TestSuite = (name: string, fn: () => void) => void;

interface SpyStats<ReturnValue = unknown, Arguments = unknown> {
    callCount: number;
    called: boolean;
    calls: Arguments[];
    returnValue: ReturnValue;
}

interface ExpectationMatchers<T = unknown> {
    toBe(expected: T): void;
    toEqual(expected: T): void;
    toDeepEqual(expected: T): void;
    toBeTruthy(): void;
    toBeFalsy(): void;
    toContain(item: unknown): void;
    toThrow(): boolean;
}

export function describe(name: string, fn: () => void): void;
export function beforeEach(fn: () => void): void;
export function test(name: string, fn: () => void): void;
export function expect<ActualValue>(actual: ActualValue): ExpectationMatchers<ActualValue>;
export function spyOn<TargetObject extends object, MethodName extends keyof TargetObject>(
    obj: TargetObject,
    method: MethodName,
    returnValue: () => unknown
): SpyStats;