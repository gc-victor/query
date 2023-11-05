use std::env;

use anyhow::{anyhow, Result};
use rustyscript::deno_core::{self, extension, op2};

use crate::controllers::utils::query_to_json::query_to_json;
use crate::sqlite::connect_db::connect_db;

#[op2]
#[string]
fn op_sqlite_query_extension(#[string] db_name: String, #[string] query: String) -> Result<String> {
    let connection = connect_db(&db_name)?;

    let statement = connection.prepare(&query);

    if statement.is_err() {
        Err(anyhow!(format!("Error: {}", statement.unwrap_err())))
    } else {
        Ok(query_to_json(statement.unwrap(), []).unwrap().to_string())
    }
}

#[op2]
#[string]
fn op_sqlite_execute_extension(
    #[string] db_name: String,
    #[string] query: String,
) -> Result<String> {
    let connection = connect_db(&db_name)?;

    let statement = connection.prepare(&query);

    if statement.is_err() {
        Err(anyhow!(format!("Error: {}", statement.unwrap_err())))
    } else {
        Ok(query_to_json(statement.unwrap(), []).unwrap().to_string())
    }
}

extension!(
    init_sqlite,
    ops = [op_sqlite_query_extension, op_sqlite_execute_extension],
    esm_entry_point = "ext:init_sqlite/init_sqlite.js",
    esm = [ dir "src/controllers/functions/ext", "init_sqlite.js" ],
);
