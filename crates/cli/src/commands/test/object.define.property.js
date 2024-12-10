const __export = (target, all) => {
    for (const name in all) {
        Object.defineProperty(target, name, {
            get: () => all[name](),
            set(v) {
                all[name] = () => v;
            },
            enumerable: true,
            configurable: true,
        });
    }
};

const obj = {
    testFn1: () => testFn1,
    testFn2: () => testFn2,
};
obj.testFn1 = () => "replaced1";
console.log(obj.testFn1()); // replaced1

const mock_imports_exports = {};
__export(mock_imports_exports, {
    testFn1: () => testFn1,
    testFn2: () => testFn2,
});
const testFn1 = () => "original1";
const testFn2 = () => "original2";
const myFunction1 = (n) => mock_imports_exports.testFn1(n);

console.log("mock_imports_exports", mock_imports_exports);
console.log("myFunction1 before replacement:", myFunction1("0")); // original1

const stats = spyOn(mock_imports_exports, "testFn1", () => "replaced1");
console.log("myFunction1 after replacement:", myFunction1("1")); // replaced1

console.log("stats.callCount", stats.callCount); // 1
console.log("stats.called", stats.called); // true
console.log("stats.calls", stats.calls); // []
console.log("stats.returnValue", stats.returnValue()); // replaced1

function spyOn(obj, method, returnValue) {
    const stats = {
        callCount: 0,
        called: false,
        calls: [],
        returnValue: null
    };

    Object.defineProperty(obj, method, {
        get: (...args) => {
            stats.callCount++;
            stats.called = true;
            stats.calls.push(...args);
            stats.returnValue = returnValue;
            return stats.returnValue;
        },
        enumerable: true,
        configurable: true,
    });

    return stats;
};
