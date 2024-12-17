import * as outerImports from './spy-imports';

export function myFunction1(c) {
    const innerImports = outerImports;
    return innerImports.testFn1(c);
}