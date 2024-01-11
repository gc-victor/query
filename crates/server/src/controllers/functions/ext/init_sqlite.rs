use std::env;

use anyhow::{anyhow, Result};
use rustyscript::deno_core::{self, extension, op2};
use serde_json::Value;

use crate::controllers::utils::bind_to_params::{bind_array_to_params, bind_object_to_params};
use crate::controllers::utils::query_to_json::query_to_json;
use crate::sqlite::connect_db::connect_db;

#[op2]
#[string]
fn op_sqlite_query_extension(
    #[string] db_name: String,
    #[string] query: String,
    #[string] params: String,
) -> Result<String> {
    let connection = connect_db(&db_name)?;

    let values: Value = serde_json::from_str(&params).unwrap();

    let (statement, params) = if values.is_object() {
        let (params, updated_query) = bind_object_to_params(values, query.to_owned())?;
        let statement = connection.prepare(&updated_query);

        (statement, params)
    } else {
        let params = bind_array_to_params(values);
        let statement = connection.prepare(&query);

        (statement, params)
    };

    match statement.is_err() {
        true => Err(anyhow!(format!("Error: {}", statement.unwrap_err()))),
        false => Ok(query_to_json(statement.unwrap(), params)
            .unwrap()
            .to_string()),
    }
}

#[op2]
#[string]
fn op_sqlite_execute_extension(
    #[string] db_name: String,
    #[string] query: String,
    #[string] params: String,
) -> Result<String> {
    let connection = connect_db(&db_name)?;

    let values: Value = serde_json::from_str(&params).unwrap();

    let (statement, params) = if values.is_object() {
        let (params, updated_query) = bind_object_to_params(values, query.to_owned())?;
        let statement = connection.prepare(&updated_query);

        (statement, params)
    } else {
        let params = bind_array_to_params(values);
        let statement = connection.prepare(&query);

        (statement, params)
    };

    match statement.is_err() {
        true => Err(anyhow!(format!("Error: {}", statement.unwrap_err()))),
        false => Ok(query_to_json(statement.unwrap(), params)
            .unwrap()
            .to_string()),
    }
}

extension!(
    init_sqlite,
    ops = [op_sqlite_query_extension, op_sqlite_execute_extension],
    esm_entry_point = "ext:init_sqlite/init_sqlite.js",
    esm = [ dir "src/controllers/functions/ext", "init_sqlite.js" ],
);
