globalThis.process = { env: JSON.parse(Deno.core.ops.op_process_extension()) };
