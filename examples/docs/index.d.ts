/// <reference lib="dom" />

declare global {
    const process: {
        env: {
            [key: string]: string | undefined;
        };
    };

    namespace JSX {
        interface Element {
            type: string;
            props: { [key: string]: unknown };
            children: unknown[];
        }

        interface IntrinsicElements {
            [elemName: string]: unknown;
        }
    }
    type ComponentChild = object | string | number | bigint | boolean | null | undefined | JSX.Element;
    type ComponentChildren = ComponentChild[] | ComponentChild;
    const Fragment: (props: JSX.Fragment) => ComponentChildren;
    const StringHTML: (input: string) => string;

    class Database {
        constructor(path: string);
        query<T>(sql: string, params?: unknown[] | Record<string, unknown>): T[];
        query_cache<T>(query: string, params: unknown[] | Record<string, unknown>, ttl: number): T;
    }

    declare module "*query:database" {
        export class Database {
            constructor(path: string);
            query<T>(sql: string, params?: unknown[] | Record<string, unknown>): T[];
            query_cache<T>(query: string, params: unknown[] | Record<string, unknown>, ttl: number): T;
        }
    }

    declare module "*query:plugin" {
        export function plugin(name: string, fn: string, input: string, options: string | null): string;
    }
    
    declare module "*query:test" {
        export function describe(name: string, fn: () => void): void;
        export function beforeEach(fn: () => void): void;
        export function test(name: string, fn: () => void): void;
        export function expect<ActualValue>(actual: ActualValue): ExpectationMatchers<ActualValue>;
        export function spyOn<TargetObject extends object, MethodName extends keyof TargetObject>(
            obj: TargetObject,
            method: MethodName,
            returnValue: () => unknown,
        ): SpyStats;
    }

    declare module "*.html" {
        const content: string;
        export default content;
    }
    
    declare module "*.svg" {
        const content: string;
        export default content;
    }

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

    interface Window {
        ReactiveComponent: typeof ReactiveComponent;
    }
}

export type {};
