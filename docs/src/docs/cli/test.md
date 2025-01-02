# Test

Query's test suite provides a simple and efficient way to write and run tests for JavaScript and TypeScript code. Inspired by Jest and Bun's test runner, it offers a familiar API and essential features to ensure your code works as intended.

### Features

- **Familiar Syntax**: Use `test`, `describe`, and `expect` functions similar to Jest.
- **Assertion Matchers**: Validate your code with a variety of matchers like `.toBe`, `.toEqual`, and more.
- **Asynchronous Testing**: Support for async tests using `async/await`.
- **Lifecycle Hooks**: Support for setup and teardown with `beforeAll`, `beforeEach`, `afterEach`, and `afterAll` hooks at both file and suite levels.
- **Spying and Mocking**: Monitor function calls with `spyOn`.
- **Command-Line Options**: Run tests with filters, watch mode, and more.

## Running Tests

The test suite is integrated into our CLI tool. You can run tests using the `test` command.

### Command-Line Usage

```bash
query test [filters] [options]
```

- `filters`: (Optional) Specify test files or directories to run. If omitted, all test files will be executed.

**Options:**

- `-s`, `--spy`: Enable function call spying for mocking (Experimental).
- `-t`, `--test-name-pattern <pattern>`: Run only tests with names matching the given pattern.
- `-w`, `--watch`: Watch for file changes and re-run tests automatically.

### Examples

- **Run All Tests**

  ```bash
  query test
  ```

- **Run Specific Test Files**

  ```bash
  query test tests/math.test.js tests/string.test.js
  ```

- **Filter Tests by Name Pattern**

  ```bash
  query test -t "addition"
  ```

- **Enable Function Spying**

  ```bash
  query test tests/**/*.test.js --spy
  ```

- **Watch for File Changes**

  ```bash
  query test tests/**/*.test.js --watch
  ```

## Writing Tests

Tests are written in JavaScript or TypeScript files using the testing functions provided by the framework.

### Importing Test Functions

You can import the test functions from `"query:test"`:

```javascript
import { test, describe, expect, spyOn } from "query:test";
```

Alternatively, you can rely on global injection if supported.

### Basic Test Structure

**Defining a Test Case**

Use the `test` function to define a test case.

```javascript
test("should add two numbers correctly", () => {
  expect(1 + 2).toBe(3);
});
```

**Grouping Tests with `describe`**

Use `describe` to group related tests together.

```javascript
describe("Math operations", () => {
  test("addition", () => {
    expect(1 + 2).toBe(3);
  });

  test("subtraction", () => {
    expect(5 - 2).toBe(3);
  });
});
```

### Asynchronous Tests

**Using `async/await`**

You can define asynchronous tests by making the test function `async`.

```javascript
test("fetch data from API", async () => {
  const data = await fetchDataFromAPI();
  expect(data).toEqual(expectedData);
});
```

## Assertions with `expect`

The `expect` function is used to assert that a value meets certain conditions. It provides several matcher methods.

### Common Matchers

- **`.toBe(expected)`**: Tests strict equality (`===`).

  ```javascript
  expect(2 + 2).toBe(4);
  ```

- **`.toEqual(expected)`**: Tests deep equality using `JSON.stringify`.

  ```javascript
  expect({ a: 1 }).toEqual({ a: 1 });
  ```

- **`.toDeepEqual(expected)`**: Tests deep equality checking nested objects.

  ```javascript
  expect({ a: { b: 2 } }).toDeepEqual({ a: { b: 2 } });
  ```

- **`.toBeTruthy()`**: Asserts that the value is truthy.

  ```javascript
  expect("non-empty string").toBeTruthy();
  ```

- **`.toBeFalsy()`**: Asserts that the value is falsy.

  ```javascript
  expect(null).toBeFalsy();
  ```

- **`.toContain(item)`**: Checks if an array contains the item.

  ```javascript
  expect([1, 2, 3]).toContain(2);
  ```
  
- **`.toMatch(pattern)`**: Tests if a string matches a regular expression or string pattern.

  ```javascript
  expect("hello world").toMatch(/world/);
  expect("hello world").toMatch("hello");
  ```

- **`.toThrow()`**: Expects the function to throw an error.

  ```javascript
  expect(() => {
    throw new Error("Error!");
  }).toThrow();
  ```

### Negating Matchers with `not`

You can negate any matcher by chaining `.not` before the matcher:

```javascript
test("not examples", () => {
  expect(1).not.toBe(2);
  expect([1, 2]).not.toContain(3);
  expect({ a: 1 }).not.toEqual({ a: 2 });
});
```

### Usage Examples

**Testing Numbers**

```javascript
test("number comparisons", () => {
  expect(10).toBe(10);
  expect(5 + 5).toEqual(10);
});
```

**Testing Strings**

```javascript
test("string comparisons", () => {
  expect("Hello, World!").toBe("Hello, World!");
  expect("Hello" + ", " + "World!").toEqual("Hello, World!");
});
```

**Testing Objects**

```javascript
test("object equality", () => {
  const obj = { a: 1, b: 2 };
  expect(obj).toEqual({ a: 1, b: 2 });
});

test("deep object equality", () => {
  const obj = { a: { b: { c: 3 } } };
  expect(obj).toDeepEqual({ a: { b: { c: 3 } } });
});
```

## Lifecycle Hooks

Query's test suite provides lifecycle hooks that allow you to run setup and teardown code at various points during test execution. These hooks can be defined at both the file level and within test suites.

### Available Hooks

| Hook         | Description                                   |
| ------------ | --------------------------------------------- |
| `beforeAll`  | Runs once before all tests in a file or suite |
| `beforeEach` | Runs before each test in a file or suite      |
| `afterEach`  | Runs after each test in a file or suite       |
| `afterAll`   | Runs once after all tests in a file or suite  |

### Hook Execution Order

When running tests, hooks execute in the following order:

1. File-level `beforeAll`
2. Suite-level `beforeAll` (if within a describe block)
3. File-level `beforeEach`
4. Suite-level `beforeEach` (if within a describe block)
5. Test execution
6. Suite-level `afterEach` (if within a describe block)
7. File-level `afterEach`
8. Suite-level `afterAll` (if within a describe block)
9. File-level `afterAll`

### Example Usage

```javascript
import { describe, beforeAll, beforeEach, afterEach, afterAll, test, expect } from "query:test";

// File-level hooks
beforeAll(() => {
  // Runs once before all tests in the file
  console.log("File beforeAll");
});

beforeEach(() => {
  // Runs before each test in the file
  console.log("File beforeEach");
});

afterEach(() => {
  // Runs after each test in the file
  console.log("File afterEach");
});

afterAll(() => {
  // Runs once after all tests in the file
  console.log("File afterAll");
});

describe("test suite", () => {
  // Suite-level hooks
  beforeAll(() => {
    // Runs once before all tests in this suite
    console.log("Suite beforeAll");
  });

  beforeEach(() => {
    // Runs before each test in this suite
    console.log("Suite beforeEach");
  });

  afterEach(() => {
    // Runs after each test in this suite
    console.log("Suite afterEach");
  });

  afterAll(() => {
    // Runs once after all tests in this suite
    console.log("Suite afterAll");
  });

  test("example test", () => {
    console.log("Test execution");
    expect(true).toBeTruthy();
  });
});
```

For the example above, the output would show the following execution order:

```
File beforeAll
Suite beforeAll
File beforeEach
Suite beforeEach
Test execution
Suite afterEach
File afterEach
Suite afterAll
File afterAll
```

### Best Practices

- Use `beforeAll` for one-time setup that is needed for all tests
- Use `beforeEach` for setup that should be fresh for each test
- Use `afterEach` to clean up after each test
- Use `afterAll` for one-time cleanup after all tests
- Keep hooks focused and minimal to prevent test interdependence
- Consider using suite-level hooks to organize related setup/teardown
- Use file-level hooks sparingly and only for truly global setup/teardown

## Spying and Mocking with `spyOn`

The `spyOn` function allows you to monitor and mock functions. It's useful for testing how functions are called and to replace real implementations with mock ones.

### Syntax

```javascript
const spy = spyOn(object, "methodName", mockImplementation);
```

- `object`: The object containing the method.
- `methodName`: The name of the method to spy on.
- `mockImplementation`: A function that replaces the original method.

### Returned Spy Object

The `spyOn` function returns an object with the following properties:

- `callCount`: Number of times the method was called.
- `called`: Boolean indicating if the method was called at least once.
- `calls`: Array of arguments from each call.
- `returnValue`: The return value from the last call.

### Example

```javascript
test("spy on object method", () => {
  const calculator = {
    add: (a, b) => a + b,
  };

  // Spy on the 'add' method
  const spy = spyOn(calculator, "add", (a, b) => a * b);

  const result = calculator.add(2, 3);

  expect(result).toBe(6); // Mock implementation multiplies instead of adds
  expect(spy.called).toBeTruthy();
  expect(spy.callCount).toBe(1);
  expect(spy.calls).toEqual([2, 3]);
  expect(spy.returnValue).toBe(6);
});
```

**Note:** Spying is currently an experimental feature and might change in future versions.

## Test Results and Reporting

After running the tests, the framework collects and reports the results.

### Output Summary

The test runner will output a summary including:

- Number of files tested.
- Total number of tests.
- Number of passed tests.
- Number of failed tests.
- Execution time.

**Example Output:**

```
Files: 2
Tests: 5
Passed: 5
Failed: 0
Time: 25ms
```

### Viewing Failed Tests

If there are failed tests, the runner will provide details about each failure.

**Example Output with Failures:**

```
File: tests/math.test.js
Failed: 1 test

Test: subtraction fails
- Expected 5 - 3 to be 3

Files: 2
Tests: 5
Passed: 4
Failed: 1
Time: 30ms
```

## Watching for File Changes

To automatically rerun tests when code changes, use the `--watch` flag.

```bash
query test --watch
```

The test runner will monitor files and rerun the relevant tests upon modification.

## Advanced Usage

### Filtering Tests by Name

Use the `--test-name-pattern` option to run only tests matching a specific pattern.

```bash
query test --test-name-pattern "addition"
```

### Running Specific Test Files

Specify the test files or directories as arguments to run only those tests.

```bash
query test tests/math.test.js
```

### Enabling Spying

Enable function spying globally with the `--spy` option.

```bash
query test --spy
```

## Best Practices

- **Name Tests Clearly**: Use descriptive names for your tests to make it easy to understand the purpose.
- **Keep Tests Focused**: Each test should check a single functionality or behavior.
- **Avoid Global State**: Ensure tests do not rely on or modify shared global state to prevent flaky tests.
