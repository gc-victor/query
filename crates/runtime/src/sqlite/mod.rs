use rquickjs::{function::Func, Ctx, Exception, Result};
use serde_json::Value;

mod connect_db;
mod functions;

use crate::utils::bind_to_params::{bind_array_to_params, bind_object_to_params};
use crate::utils::query_to_json::query_to_json;

use self::connect_db::connection;

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    globals.set("___sqlite_query", Func::from(sqlite_query))?;

    Ok(())
}

fn sqlite_query(ctx: Ctx<'_>, db_name: String, query: String, params: String) -> Result<String> {
    let connection = match connection(&db_name) {
        Ok(v) => Ok(v),
        Err(e) => Err(Exception::throw_syntax(&ctx, &format!("Error: {}", e))),
    }?;

    let values: Value = serde_json::from_str(&params).unwrap();

    let (statement, params) = if values.is_object() {
        let (params, updated_query) = match bind_object_to_params(values, query.to_owned()) {
            Ok(v) => Ok(v),
            Err(e) => Err(Exception::throw_syntax(&ctx, &format!("Error: {}", e))),
        }?;
        let statement = connection.prepare(&updated_query);

        (statement, params)
    } else {
        let params = bind_array_to_params(values);
        let statement = connection.prepare(&query);

        (statement, params)
    };

    match statement.is_err() {
        true => Err(Exception::throw_syntax(
            &ctx,
            &format!("Error: {}", statement.unwrap_err()),
        )),
        false => Ok(query_to_json(statement.unwrap(), params)
            .unwrap()
            .to_string()),
    }
}
