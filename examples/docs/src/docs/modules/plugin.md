# Plugin Module

Query's plugin system allows you to extend functionality using WebAssembly (WASM) plugins. The system is built on top of Extism, enabling plugins to be written in multiple languages including Rust, C, C++, Go, TypeScript, and more.

## Basic Usage

```javascript
import { plugin } from 'query:plugin';

// Execute a plugin function
const result = await plugin(
    "plugin_name.wasm",  // Plugin file name
    "function_name",     // Function to call
    "input_data",        // String input
    null                 // Optional configuration
);
```

## API Reference

### plugin(name, function, input, options?)

Executes a plugin function with the specified parameters.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | string | Yes | Plugin file name (e.g., "plugin_example.wasm") |
| function | string | Yes | Name of the function to execute |
| input | string | Yes | Input data for the plugin function |
| options | PluginOptions | No | Plugin configuration options |

#### Plugin Options

```typescript
interface PluginOptions {
    memory?: {
        maximum?: number;  // Maximum memory pages (64KB each)
        requested?: number;  // Requested initial memory pages
    };
    allowed_hosts?: string[] | null;  // Allowed hosts for HTTP requests
    allowed_paths?: Record<string, string> | null;  // Filesystem path mappings
    config?: Record<string, unknown>;  // Plugin-specific configuration
    timeout?: number;  // Execution timeout in milliseconds
}
```

For more information about the Query Plugin System, refer to the [Query Plugin System](/docs/server/plugin-system.html).
