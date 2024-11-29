use std::sync::OnceLock;
use std::time::{Duration, SystemTime};

use mini_moka::sync::Cache;
use rquickjs::{function::Func, Ctx, Exception, Result};
use serde_json::Value;
use tracing::instrument;

pub mod connect_db;
mod functions;

use crate::utils::bind_to_params::{bind_array_to_params, bind_object_to_params};
use crate::utils::query_to_json::query_to_json;

use self::connect_db::connection;

static CACHE: OnceLock<Cache<String, (SystemTime, String)>> = OnceLock::new();

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    globals.set("___sqlite_query", Func::from(sqlite_query))?;

    Ok(())
}

#[instrument(err, skip(ctx, params))]
fn sqlite_query(
    ctx: Ctx<'_>,
    db_name: String,
    query: String,
    params: String,
    ttl: u64,
) -> Result<String> {
    let cache = CACHE.get_or_init(|| Cache::new(1000));
    let cache_key = format!("{}-{}-{}-{}", db_name, query, params, ttl);

    if ttl > 0 && query.starts_with("SELECT") {
        if let Some((timestamp, result)) = cache.get(&cache_key) {
            if let Ok(elapsed) = SystemTime::now().duration_since(timestamp) {
                if elapsed < Duration::from_millis(ttl) {
                    return Ok(result.clone());
                }
            }
        }
    }

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

    let result = match statement {
        Err(err) => Err(Exception::throw_syntax(&ctx, &format!("Error: {}", err))),
        Ok(mut statement) => {
            if query.to_uppercase().starts_with("INSERT") {
                statement
                    .insert(params)
                    .map_err(|e| Exception::throw_syntax(&ctx, &format!("Insert error: {}", e)))
                    .map(|result| format!("{{\"rowid\": {}}}", result))
            } else if query.to_uppercase().starts_with("SELECT") {
                query_to_json(statement, params)
                    .map_err(|e| {
                        Exception::throw_syntax(&ctx, &format!("Query result to JSON error: {}", e))
                    })
                    .map(|result| result.to_string())
            } else {
                statement
                    .execute(params)
                    .map_err(|e| {
                        Exception::throw_syntax(&ctx, &format!("Update or delete error: {}", e))
                    })
                    .map(|result| format!("{{\"changes\": {}}}", result))
            }
        }
    };

    let result_string = result?;

    if ttl > 0 && query.starts_with("SELECT") {
        cache.insert(cache_key, (SystemTime::now(), result_string.clone()));
    }
    Ok(result_string)
}

pub fn query_cache_invalidate() {
    let cache = CACHE.get_or_init(|| Cache::new(1000));
    cache.invalidate_all();
}
