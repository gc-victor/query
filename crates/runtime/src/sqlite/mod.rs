use std::sync::OnceLock;
use std::time::{Duration, SystemTime};

use mini_moka::sync::Cache;
use rquickjs::{function::Func, Ctx, Exception, Result};
use serde_json::Value;
use tracing::instrument;

pub mod connect_db;
mod functions;

use crate::utils::bind_to_params::{bind_array_to_params, bind_named_params};
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

    if ttl > 0 && is_select(&query) {
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
        Err(e) => Err(Exception::throw_syntax(
            &ctx,
            &format!("Database connection error: {}", e),
        )),
    }?;

    let values: Value = serde_json::from_str(&params).unwrap();

    let mut stmt = match connection.prepare(&query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(Exception::throw_syntax(
                &ctx,
                &format!("Statement preparation error: {}", e),
            ))
        }
    };

    let result = if is_select(&query) {
        execute_select(&mut stmt, values, &ctx)
    } else if query.to_uppercase().starts_with("INSERT") {
        execute_insert(&mut stmt, values, &ctx)
    } else {
        execute_other(&mut stmt, values, &ctx)
    }?;

    if ttl > 0 && is_select(&query) {
        cache.insert(cache_key, (SystemTime::now(), result.clone()));
    }

    Ok(result)
}

fn is_select(query: &str) -> bool {
    regex::Regex::new(r"^(?i)SELECT|^(?i)WITH RECURSIVE.*AS \(([\s\S]+?)\)\s*SELECT")
        .unwrap()
        .is_match(query)
}

fn execute_select(stmt: &mut rusqlite::Statement, params: Value, ctx: &Ctx) -> Result<String> {
    let result = if params.is_object() {
        let params_bound = bind_named_params(params);
        let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
            .iter()
            .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
            .collect::<Vec<_>>();
        query_to_json(stmt, params)
    } else {
        query_to_json(stmt, bind_array_to_params(params))
    };

    result
        .map_err(|e| Exception::throw_syntax(ctx, &format!("SELECT error: {}", e)))
        .map(|v| v.to_string())
}

fn execute_insert(stmt: &mut rusqlite::Statement, params: Value, ctx: &Ctx) -> Result<String> {
    let result = if params.is_object() {
        let params_bound = bind_named_params(params);
        let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
            .iter()
            .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
            .collect::<Vec<_>>();

        match stmt.insert(params) {
            Ok(rowid) => Ok(serde_json::json!({ "rowid": rowid })),
            Err(rusqlite::Error::StatementChangedRows(0)) => {
                Ok(serde_json::json!({ "rowid": 0 }))
            }
            Err(e) => Err(e),
        }
    } else {
        let params = bind_array_to_params(params);
        match stmt.insert(params.clone()) {
            Ok(rowid) => Ok(serde_json::json!({ "rowid": rowid })),
            Err(rusqlite::Error::StatementChangedRows(0)) => {
                Ok(serde_json::json!({ "rowid": 0 }))
            }
            Err(e) => Err(e),
        }
    };

    result
        .map_err(|e| Exception::throw_syntax(ctx, &format!("INSERT error: {}", e)))
        .map(|v| v.to_string())
}

fn execute_other(stmt: &mut rusqlite::Statement, params: Value, ctx: &Ctx) -> Result<String> {
    let result = if params.is_object() {
        let params_bound = bind_named_params(params);
        let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
            .iter()
            .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
            .collect::<Vec<_>>();

        stmt.execute(params)
    } else {
        stmt.execute(bind_array_to_params(params))
    };

    result
        .map_err(|e| Exception::throw_syntax(ctx, &format!("Statement execution error: {}", e)))
        .map(|changes| {
            serde_json::json!({ "changes": changes }).to_string()
        })
}

pub fn query_cache_invalidate() {
    let cache = CACHE.get_or_init(|| Cache::new(1000));
    cache.invalidate_all();
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use serde_json::json;

    use crate::test_utils::utils::with_js_runtime;

    use super::*;

    #[test]
    fn test_is_select() {
        assert!(is_select("SELECT * FROM table"));
        assert!(is_select(
            "WITH RECURSIVE cte AS (SELECT * FROM table) SELECT * FROM cte"
        ));
        assert!(!is_select("INSERT INTO table VALUES (1)"));
        assert!(!is_select("UPDATE table SET col = 1"));
    }

    #[tokio::test]
    async fn test_sqlite_query_select() {
        with_js_runtime(|ctx| {
            init(&ctx)?;

            let result = sqlite_query(
                ctx.clone(),
                ":memory:".to_string(),
                "SELECT 1 as num".to_string(),
                "[]".to_string(),
                0,
            )?;

            assert_eq!(result, r#"[{"num":1}]"#);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn test_sqlite_query_with_cache() {
        use std::thread;
        use std::time::Duration;

        with_js_runtime(|ctx| {
            init(&ctx)?;

            let result1 = sqlite_query(
                ctx.clone(),
                ":memory:".to_string(),
                "SELECT 1 as num".to_string(),
                "[]".to_string(),
                1000, // 1 second TTL
            )?;

            let result2 = sqlite_query(
                ctx.clone(),
                ":memory:".to_string(),
                "SELECT 1 as num".to_string(),
                "[]".to_string(),
                1000,
            )?;

            assert_eq!(result1, result2);

            thread::sleep(Duration::from_millis(1100));

            let result3 = sqlite_query(
                ctx.clone(),
                ":memory:".to_string(),
                "SELECT 1 as num".to_string(),
                "[]".to_string(),
                1000,
            )?;

            assert_eq!(result1, result3);
            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn test_sqlite_query_named_params() {
        with_js_runtime(|ctx| {
            init(&ctx)?;

            let result = sqlite_query(
                ctx.clone(),
                ":memory:".to_string(),
                "SELECT :value as num".to_string(),
                r#"{":value": 42}"#.to_string(),
                0,
            )?;

            assert_eq!(result, r#"[{"num":42}]"#);
            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn test_execute_select() {
        with_js_runtime(|ctx| {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
                .unwrap();
            conn.execute("INSERT INTO test (name) VALUES (?1)", ["Alice"])
                .unwrap();

            let mut stmt = conn.prepare("SELECT * FROM test WHERE name = ?").unwrap();
            let params = json!(["Alice"]);
            let result = execute_select(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"[{"id":1,"name":"Alice"}]"#);

            let mut stmt = conn
                .prepare("SELECT * FROM test WHERE name = :name")
                .unwrap();
            let params = json!({ ":name": "Alice" });
            let result = execute_select(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"[{"id":1,"name":"Alice"}]"#);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn test_execute_insert() {
        with_js_runtime(|ctx| {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
                .unwrap();

            let mut stmt = conn.prepare("INSERT INTO test (name) VALUES (?)").unwrap();
            let params = json!(["Bob"]);
            let result = execute_insert(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"{"rowid":1}"#);

            let mut stmt = conn
                .prepare("INSERT INTO test (name) VALUES (:name)")
                .unwrap();
            let params = json!({ ":name": "Charlie" });
            let result = execute_insert(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"{"rowid":2}"#);

            let mut stmt = conn
                .prepare("INSERT INTO test (id, name) VALUES (?1, ?2)")
                .unwrap();
            let params = json!([1, "Dave"]); // ID 1 already exists
            let result = execute_insert(&mut stmt, params, &ctx);
            assert!(result.is_err());

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn test_execute_other() {
        with_js_runtime(|ctx| {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
                .unwrap();
            conn.execute("INSERT INTO test (name) VALUES (?1)", ["Eve"])
                .unwrap();

            let mut stmt = conn
                .prepare("UPDATE test SET name = ? WHERE id = ?")
                .unwrap();
            let params = json!(["Eva", 1]);
            let result = execute_other(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"{"changes":1}"#);

            let mut stmt = conn
                .prepare("UPDATE test SET name = :name WHERE id = :id")
                .unwrap();
            let params = json!({ ":name": "Eve", ":id": 1 });
            let result = execute_other(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"{"changes":1}"#);

            let mut stmt = conn.prepare("DELETE FROM test WHERE id = ?").unwrap();
            let params = json!([1]);
            let result = execute_other(&mut stmt, params, &ctx).unwrap();
            assert_eq!(result, r#"{"changes":1}"#);
            
            Ok(())
        })
        .await;
    }
}
