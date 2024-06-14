const funFunFunction = `globalThis.___handleRequest = async (req) => {
    try {
        const formData = await req.formData();
        const email = formData.get("email");
        const password = formData.get("password");
        const field = formData.get("field");
        const duplicated = formData.getAll("duplicated");
        const arr = formData.getAll("arr");
        const json = formData.getAll("json");
        const list = formData.getAll("list");
        const str = \`email: \${email} | password: \${password} | file: \${field.name} - \${field.type} - \${field.size} | duplicated: \${duplicated}\ | duplicatedArray: \${Array.isArray(duplicated)}\ | arr: \${arr}\ | arrArray: \${Array.isArray(arr)}\ | json: \${json}\ | jsonArray: \${Array.isArray(json)}\ | list: \${list} | listArray: \${Array.isArray(list)}\`;

        return new Response(str, {
            status: 200,
            headers: {
                "content-type": "text/plain;charset=UTF-8",
            },
        });
    } catch (e) {
        console.log(e.message);

        return new Response(e.message, {
            status: 500,
            headers: {
                "content-type": "text/plain;charset=UTF-8",
            },
        });
    }
};`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(funFunFunction)).toString()}]`);
