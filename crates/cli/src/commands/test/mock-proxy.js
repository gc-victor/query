var ___mockRegistry = {};
var ___addMock = (functionName, returnValue) => {
  ___mockRegistry[functionName] = returnValue;
};
var ___clearMocks = () => {
    ___mockRegistry = {};
};

Object.defineProperty = new Proxy(Object.defineProperty, {
  apply(target, thisArg, args) {
    const [obj, prop, descriptor] = args;

    return ___mockRegistry[descriptor.value] || target.apply(thisArg, args);
  }
});

var __name = (target, value) => Object.defineProperty(target, "name", { value, configurable: true });

// Example usage
const obj = {};
__name(obj, "mockThis");

___addMock(
  "mockThis",
  { mocked: true }
);

console.log(__name(obj, "mockThis")); // { mocked: true }
console.log("___mockRegistry", ___mockRegistry);
