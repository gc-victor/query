use anyhow::Result;
use rustyscript::deno_core::{self, extension, op2};
use serde_json::{json, Value};

#[op2]
#[string]
fn op_process_extension() -> Result<String> {
    let env_vars: Vec<(String, String)> = std::env::vars().collect();

    let mut obj = json!({});

    for (key, value) in env_vars {
        obj[key] = Value::String(value);
    }

    Ok(obj.to_string())
}

extension!(
    init_process,
    ops = [op_process_extension],
    esm_entry_point = "ext:init_process/init_process.js",
    esm = [ dir "src/controllers/functions/ext", "init_process.js" ],
);
