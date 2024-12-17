import { describe, expect, test, spyOn } from 'query:test';

import * as imports from "./spy-imports";
import { myFunction1 } from "./spy-case";

describe("Integration", () => {
    test("should execute mock function with mocked value", () => {
        const stats = spyOn(imports, "testFn1", () => "replaced1");

        const result = myFunction1("ooo");
        const _ = myFunction1("iii");

        expect(stats.callCount).toBe(2);
        expect(stats.called).toBe(true);
        expect(stats.calls).toEqual(["ooo", "iii"]);
        expect(stats.returnValue).toBe("replaced1");
        expect(result).toBe("replaced1");
    });
});
