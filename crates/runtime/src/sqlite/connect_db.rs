use std::{env, fs, path::Path};

use anyhow::Result;
use rusqlite::{limits::Limit, Connection};

use crate::sqlite::functions::{
    _base64_decode_function, _base64_encode_function, _not_allowed_function, _regexp_function,
    _token_function, _uuid_function, _valid_json_function,
};

pub fn connection(db_name: &str) -> Result<Connection> {
    let path = env::var("QUERY_SERVER_DBS_PATH").unwrap_or("/mnt/dbs".to_string());

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    let conn = Connection::open(format!("{}/{}", &path, db_name))?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0);

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
