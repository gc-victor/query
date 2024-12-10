import * as mockImports from './mock-imports';

export function myFunction1(c) {
    const imports = mockImports;
    return imports.testFn1(c);
}