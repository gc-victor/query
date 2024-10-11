# Plugins System

Query plugins can be written in WebAssembly (WASM), allowing the use of languages like Rust, C, C++, Go, TypeScript, and more thanks to [Extism](https://extism.org), a cross-language framework for building with WASM used by Query to build plugins.

Extism's provides [Plug-in Development Kits (PDKs)](https://extism.org/docs/concepts/pdk) and [Host Software Development Kits (SDKs)](https://extism.org/docs/concepts/host-sdk). The PDKs are used to build plugins, while the Host SDKs are used to run plugins. Query implements the Host SDKs to run the plugins, so you don't have to worry about it. You only have to build the plugins using the PDKs. Some of the PDKs are available in the following languages:

- [Rust](https://github.com/extism/rust-pdk)
- [JavaScript/TypeScript](https://github.com/extism/js-pdk)
- [Go](https://github.com/extism/go-pdk)
- [C](https://github.com/extism/c-pdk)
- [AssemblyScript](https://github.com/extism/assemblyscript-pdk)

You can find a full example of a Rust plugin in the <https://github.com/gc-victor/query-plugin-argon2> repository.

## How to Use a Plugin

### Install a WASM Plugin

To use a plugin, you have to install it using the `query plugin install` command. The plugin should be a released WASM file in a GitHub repository. It will download the WASM file and store it in the `plugins` folder and store the plugin information in the `.query/plugins.toml` files. The plugin should be in the format of `query-plugin-*.wasm`, where "*" is the topic of the plugin.

You can place a plugin in the `plugins` folder and use the `query plugin push [PATH]` command to push it to the server.

### Use a WASM Plugin

To use a plugin, you have to import it in your function and use it. We recommend to create a module to wrap the plugin and use it in your functions.

The plugin should have a function called `plugin` that receives the following parameters:

- `name`: The name of the plugin. Ex. `plugin_argon2.wasm`
- `function`: The function name to execute in the plugin. Ex. `hash`
- `input`: The input to use in the function. It should be a string. Ex. `password`
- `options`: The options to use in the function. It should be a JSON stringify or `null`.
  - `memory`: Describes the limits on the memory the plugin may be allocated.
  - `allowed_hosts`: An optional set of hosts this plugin can communicate with. This only has an effect if the plugin makes HTTP requests. Note: if left empty then no hosts are allowed and if `null` then all hosts are allowed.
  - `allowed_paths`: An optional set of mappings between the host's filesystem and the paths a plugin can access. This only has an effect if the plugin is provided with WASI capabilities. Note: if left empty or `null`, then no file access is granted.
  - `config`: The "config" key is a free-form map that can be passed to the plugin. A plugin author must know the arbitrary data this map may contain, so your own documentation should include some information about the "config" passed in.
  - `timeout`: Set `timeout_ms`, which will interrupt a plugin function's execution if it meets or exceeds this value. When an interrupt is made, the plugin will not be able to recover and continue execution.

Example:

```js
// src/plugins/argon2.js
import { plugin } from 'query:plugin';

export const argon2 = {
  hash: (password) => plugin("plugin_argon2.wasm", "hash", "password", null),
  verify: (password, hash) => plugin("plugin_argon2.wasm", "verify", JSON.stringify({password, hash}), null) == "true"
};
```

```js
// src/functions/get.index.js
import { argon2 } from '../plugins/argon2';

export async function handleRequest(req) {
    const password = "password";
    const hash = await argon2.hash(password);
    const isValid = argon2.verify(password, hash);

    return new Response(isValid ? `✔️` : `❌`, {
      status: 200,
      headers: {
          "Content-Type": "application/json",
      },
  });
}
```
