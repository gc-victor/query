use std::{env, fs, path::Path};

use anyhow::Result;
use rusqlite::{limits::Limit, trace::{TraceEvent, TraceEventCodes}, Connection};
use tracing::info;

use crate::{
    constants::{
        DB_ASSET_NAME, DB_CACHE_INVALIDATION_NAME, DB_CONFIG_NAME, DB_FUNCTION_NAME, DB_PLUGIN_NAME,
    },
    env::Env,
};

use super::functions::{
    _base64_decode_function, _base64_encode_function, _not_allowed_function, _regexp_function,
    _token_function, _uuid_function, _valid_json_function,
};

pub fn connect_config_db() -> Result<Connection> {
    let conn = connection(DB_CONFIG_NAME)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;

    Ok(conn)
}

pub fn connect_cache_invalidation_db() -> Result<Connection> {
    let conn = connection(DB_CACHE_INVALIDATION_NAME)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;

    Ok(conn)
}

pub(crate) fn connect_asset_db() -> Result<Connection> {
    let conn = connection(DB_ASSET_NAME)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;

    Ok(conn)
}

pub fn connect_function_db() -> Result<Connection> {
    let conn = connection(DB_FUNCTION_NAME)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;

    Ok(conn)
}

pub fn connect_plugin_db() -> Result<Connection> {
    let conn = connection(DB_PLUGIN_NAME)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;

    Ok(conn)
}

pub fn connect_db(db_name: &str) -> Result<Connection> {
    let conn = connection(db_name)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 1)?;

    Ok(conn)
}

pub fn connection(db_name: &str) -> Result<Connection> {
    let path = Env::dbs_path();

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    let mut conn = Connection::open(format!("{}/{}", &path, db_name))?;

    let has_enable_query_tracing = env::var("QUERY_SERVER_ENABLE_QUERY_TRACING")
        .map(|val| val == "true")
        .unwrap_or(false);
    if has_enable_query_tracing {
        enable_query_tracing(&mut conn)?;
    }

    // Core pragmas
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "temp_store", "MEMORY")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    // Performance pragmas
    conn.pragma_update(None, "mmap_size", "30000000000")?;
    conn.pragma_update(None, "cache_size", -32000)?;
    conn.pragma_update(None, "busy_timeout", 5000)?;

    _base64_decode_function(&conn)?;
    _base64_encode_function(&conn)?;
    _not_allowed_function(&conn)?;
    _regexp_function(&conn)?;
    _token_function(&conn)?;
    _uuid_function(&conn)?;
    _valid_json_function(&conn)?;

    Ok(conn)
}

pub fn enable_query_tracing(conn: &mut Connection) -> Result<()> {
    conn.trace_v2(TraceEventCodes::SQLITE_TRACE_STMT, Some(|event| {
        let TraceEvent::Stmt(stmt_info, _) = event else { return; };
        let statement = stmt_info.expanded_sql().unwrap_or_else(|| stmt_info.sql().to_string());

        if statement.starts_with("PRAGMA") || statement.starts_with("SELECT version FROM cache_invalidation") {
            return;
        }
        info!("Statement: {}", statement);
    }));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::sqlite::create_config_db::create_config_db;

    use super::*;

    fn before(path: &str) {
        env::set_var("QUERY_SERVER_ADMIN_EMAIL", "admin@admin.com");
        env::set_var("QUERY_SERVER_ADMIN_PASSWORD", "abcdefg");
        env::set_var("QUERY_SERVER_TOKEN_SECRET", "secret");
        env::set_var("QUERY_SERVER_DBS_PATH", path);

        create_config_db();
    }

    struct AfterConnectDb;

    const PATH_AFTER_CONNECT_DB: &str = "../../.tests/after_connect_db";

    impl Drop for AfterConnectDb {
        fn drop(&mut self) {
            fs::remove_dir_all(PATH_AFTER_CONNECT_DB).unwrap();
        }
    }

    #[test]
    fn test_connect_db() {
        let _after = AfterConnectDb;

        before(PATH_AFTER_CONNECT_DB);

        let conn = connect_db("test.db").unwrap();

        assert!(conn.is_autocommit());
    }

    struct AfterConnectConfigDb;

    const PATH_AFTER_CONNECT_CONFIG_DB: &str = "../../.tests/after_connect_config_db";

    impl Drop for AfterConnectConfigDb {
        fn drop(&mut self) {
            fs::remove_dir_all(PATH_AFTER_CONNECT_CONFIG_DB).unwrap();
        }
    }

    #[test]
    fn test_connect_config_db() {
        let _after = AfterConnectConfigDb;

        before(PATH_AFTER_CONNECT_CONFIG_DB);

        let conn = connect_config_db().unwrap();

        assert!(conn.is_autocommit());
    }
}
