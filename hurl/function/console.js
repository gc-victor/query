/*

Logs output:

Array: [1,"1",[[[[[1,"1"]]]]]]
BigInt: 1
Bool: true
Constructor: function anonymous(
) {

}
Exception: An error occurred!
    at s (/console::get:26:202)
    at ___handleRequestWrapper (/console::get:78:24)
    at ___handleResponse (js/handle-response:2:28)
Float: 1.1
Function: function(){}
Int: 1
Null:: null
Object: {"a":1,"b":"1","c":{"a":{"a":{"a":1,"b":"1"}}}}
Promise: Promise {}
Proxy: {"a":1}
String: a
Symbol: Symbol(a)
Undefined:
Circular: [1,"[Circular]"]
Substitutions: Hello, world! 1 1 1.1 {"a":1} {"b":2} css:'' string

*/

const consoleFunction = `globalThis.___handleRequest = async (req) => {
    try {
        console.log("Array:", [1, "1", [[[[[1, "1"]]]]]]);
        console.log("BigInt:", BigInt(1));
        console.log("Bool:", true);
        console.log("Constructor:", new Function());
        console.log("Exception:", new Error("An error occurred!"));
        console.log("Float:", 1.1);
        console.log("Function:", function myFunction() {});
        console.log("Int:", 1);
        console.log("Null::", null);
        console.log("Object:", { a: 1, b: "1", c: { a: { a: { a: 1, b: "1" } } } });
        console.log("Promise:", Promise.resolve(1));
        console.log("Proxy:", new Proxy({ a: 1 }, {}));
        console.log("String:", "a");
        console.log("Symbol:", Symbol("a"));
        console.log("Undefined:", undefined);

        const a = [1];
        a[1] = a;
        console.log("Circular:", a);

        console.log("Substitutions (s, d): Hello, %s! You have %d new messages.", "User", 5);
        console.log("Substitution (f): Pi is approximately %f", Math.PI);
        console.log("Substitution (o): Here's an object: %o", { name: "John", age: 30 });
        console.log("Substitution (c): %cThis text is not styled!", "color: blue; font-size: 20px;");
        console.log(
            "Substitutions: Hello, %s! %d %i %f %o %O css:'%c' %s",
            "world",
            1,
            1,
            1.1,
            { a: 1 },
            { b: 2 },
            "color: red;",
            "string",
        );

        return new Response("Done!", {
            headers: { "content-type": "text/plain" },
        });
    } catch (e) {
        return new Response(e.message + "\\n" + (e.stack || ""), {
            status: 500,
            headers: { "content-type": "text/plain" },
        });
    }
};`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(consoleFunction)).toString()}]`);
