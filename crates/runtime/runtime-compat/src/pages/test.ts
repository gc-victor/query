import Tests from "./vendor/tests.ts";
import type { Test, TestResult } from "./vendor/types";
// @ts-ignore
import setup from "./vendor/harness.js";

declare global {
  // eslint-disable-next-line no-var
  var bcd: {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    addTest: (ident: string, tests: any, exposure: string[]) => void;
    addInstance: (
      resource: string,
      code: string,
      options?: { callback: boolean },
    ) => void;
    go: (
      callback: (done: TestResult[]) => void,
      resourceCount?: number,
    ) => void;
  };
}

export interface TestConfig {
  __resources: unknown;
  [name: `${string}.${string}`]: Test;
}

export function runTests(
  tests: TestConfig,
  ignoreApis?: Array<string>,
): Promise<Array<TestResult>> {
  setup(globalThis);
  // The incoming types are problematic
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const testCases = new Tests({ tests: tests as any, httpOnly: false });

  const resourcesNeeded = new Set<string>();

  const allTests = [
    ...testCases.getTests("javascript.builtins"),
    ...testCases.getTests("api", "Window", ignoreApis),
    ...testCases.getTests("webassembly", "Window"),
  ];

  for (const test of allTests) {
    globalThis.bcd.addTest(test.ident, test.tests, test.exposure);
    for (const resource of test.resources) {
      resourcesNeeded.add(resource);
    }
  }

  for (const resource of resourcesNeeded) {
    const instance = testCases.resources[resource];
    if (instance.type === "instance") {
      if (
        instance.dependencies &&
        !instance.dependencies.every((dep: string) => resourcesNeeded.has(dep))
      ) {
        console.log("missing dependencies", instance.dependencies);
      }
      globalThis.bcd.addInstance(resource, instance.src, {
        callback: instance.callback,
      });
    }
  }

  return new Promise((resolve) =>
    globalThis.bcd.go(resolve, resourcesNeeded.size),
  );
}
