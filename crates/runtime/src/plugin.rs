use std::{collections::HashMap, path::PathBuf, time::Duration};

use extism::{Manifest, Plugin as PluginExtism, Wasm};
use extism_manifest::MemoryOptions;
use rquickjs::{function::Func, Ctx, Exception, Result};
use rusqlite::{named_params, Row};
use serde::{Deserialize, Serialize};

use crate::sqlite::connect_db::connection;

// NOTE: It is defined too in crates/server/src/constants.rs
static DB_NAME: &str = "query_plugin.sql";

struct Plugin {
    hash: String,
    data: String,
}

// @see: https://extism.org/docs/concepts/manifest/
#[derive(Debug, Serialize, Deserialize)]
struct PluginConfig {
    // Describes the limits on the memory the plugin may be allocated.
    memory: Option<MemoryOptions>,
    // An optional set of hosts this plugin can communicate with.
    // This only has an effect if the plugin makes HTTP requests.
    // Note: if left empty then no hosts are allowed and if `null` then all hosts are allowed.
    allowed_hosts: Option<Vec<String>>,
    // An optional set of mappings between the host's filesystem and the paths a plugin can access.
    // This only has an effect if the plugin is provided with WASI capabilities.
    // Note: if left empty or `null`, then no file access is granted.
    allowed_paths: Option<HashMap<PathBuf, PathBuf>>,
    //  The "config" key is a free-form map that can be passed to the plugin.
    //  A plugin author must know the arbitrary data this map may contain, so your own documentation should include some information about the "config" passed in.
    config: Option<HashMap<String, String>>,
    // Set `timeout_ms`, which will interrupt a plugin function's execution if it meets or
    // exceeds this value. When an interrupt is made, the plugin will not be able to recover and
    // continue execution.
    timeout: Option<u64>,
}

fn plugin(
    ctx: Ctx<'_>,
    name: String,
    fn_name: String,
    input: String,
    options: Option<String>,
) -> Result<String> {
    let conn = match connection(DB_NAME) {
        Ok(v) => Ok(v),
        Err(e) => Err(Exception::throw_syntax(&ctx, &format!("Error: {}", e))),
    }?;
    let row_to_plugin = |row: &Row| -> std::result::Result<Plugin, rusqlite::Error> {
        let data: String = row.get(0)?;
        let hash: String = row.get(1)?;

        Ok(Plugin { data, hash })
    };
    let result = match conn.query_row(
        "SELECT data, hash FROM plugin WHERE name = :name",
        named_params! { ":name": name },
        row_to_plugin,
    ) {
        Ok(v) => Ok(v),
        Err(e) => Err(Exception::throw_syntax(&ctx, &format!("Error: {}", e))),
    }?;

    let config: PluginConfig = match options {
        Some(v) => match serde_json::from_str(&v) {
            Ok(v) => Ok(v),
            Err(e) => Err(Exception::throw_syntax(&ctx, &format!("Error: {}", e))),
        }?,
        None => PluginConfig {
            memory: None,
            allowed_hosts: None,
            allowed_paths: None,
            config: None,
            timeout: None,
        },
    };
    let wasm = Wasm::data(result.data)
        .with_name(&name)
        .with_hash(result.hash);

    let mut manifest = Manifest::new([wasm]);

    if let Some(memory) = config.memory {
        manifest = manifest.with_memory_options(memory);
    }

    if let Some(allowed_hosts) = config.allowed_hosts {
        manifest = manifest.with_allowed_hosts(allowed_hosts.into_iter());
    }

    if let Some(allowed_paths) = config.allowed_paths {
        manifest = manifest.with_allowed_paths(allowed_paths.into_iter());
    }

    if let Some(config) = config.config {
        manifest = manifest.with_config(config.into_iter());
    }

    if let Some(timeout) = config.timeout {
        manifest = manifest.with_timeout(Duration::from_millis(timeout));
    }

    let mut plugin = PluginExtism::new(&manifest, [], true).unwrap();
    let res = plugin.call::<&str, &str>(fn_name, &input).unwrap();

    Ok(res.to_string())
}

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    globals.set("___plugin", Func::from(plugin))?;

    Ok(())
}
