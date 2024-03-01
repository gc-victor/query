globalThis.argon2 = {
    hash: (password) => Deno.core.ops.op_argon2_hash_extension(password),
    verify: (password, hash) => Deno.core.ops.op_argon2_verify_extension(password, hash)
};
