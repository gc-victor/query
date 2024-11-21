use hyper::{body::Incoming, Method, Request, Response};
use rusqlite::{Connection, Error, ParamsFromIter, Statement};
use serde::Deserialize;

use anyhow::Result;
use serde_json::{json, Value};
use tracing::instrument;

use crate::{
    constants::DB_CONFIG_NAME,
    controllers::utils::{
        bind_to_params::{bind_array_to_params, bind_object_to_params},
        body::{Body, BoxBody},
        get_query_string::get_query_string,
        get_token::get_token,
        http_error::{bad_request, internal_server_error, not_found, HttpError},
        responses::ok,
        statement_to_vec::statement_to_vec,
        validate_is_admin::is_admin,
        validate_token::validate_token,
        validate_write::validate_write,
    },
    sqlite::connect_db::connect_db,
};

#[derive(Deserialize)]
struct QueryOptions {
    pub db_name: String,
    pub params: Option<Value>,
    pub query: String,
}

#[instrument(err(Debug), skip(req))]
pub async fn query(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match (req.method(), segments) {
        (&Method::GET, ["query"]) => {
            let token = get_token(req.headers().to_owned())?;

            // IMPORTANT! don't remove this validation
            validate_token(&token)?;

            let db_name = match get_query_string(req, "db_name") {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;
            let query = match get_query_string(req, "query") {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            // IMPORTANT! don't remove this validation
            if !is_select(&query) && !validate_write(&token)? {
                return Err(bad_request(
                    "Token without write permission tried to write to the database".to_string(),
                ));
            }

            // IMPORTANT! don't remove this validation
            if !is_admin(&token)? && db_name == DB_CONFIG_NAME {
                return Err(bad_request(
                    "Can't query the config database without being admin".to_string(),
                ));
            }

            if !is_select(&query) {
                return Err(bad_request(
                    "GET requests only allows read queries".to_string(),
                ));
            }

            let params = match get_query_string(req, "params") {
                Ok(p) => Some(serde_json::from_str(&p).unwrap()),
                Err(_) => None,
            };

            let conn = connect_db(&db_name)?;

            match query_controller(&conn, &query, params) {
                Ok(s) => match ok(s) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }

        (&Method::POST, ["query"]) => {
            let token = get_token(req.headers().to_owned())?;

            // IMPORTANT! don't remove this validation
            validate_token(&token)?;

            let body = Body::to_string(req.body_mut()).await?;

            let QueryOptions {
                db_name,
                params,
                query,
            } = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(internal_server_error(e.to_string())),
            }?;

            // IMPORTANT! don't remove this validation
            if !is_select(&query) && !validate_write(&token)? {
                return Err(bad_request(
                    "Token without write permission tried to write to the database".to_string(),
                ));
            }

            // IMPORTANT! don't remove this validation
            if !is_admin(&token)? && db_name == DB_CONFIG_NAME {
                return Err(bad_request(
                    "Can't query the config database without being admin".to_string(),
                ));
            }

            let conn = connect_db(&db_name)?;

            match query_controller(&conn, &query, params) {
                Ok(s) => match ok(s) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }

        _ => Err(not_found()),
    }
}

#[instrument(err(Debug), skip(conn, params))]
fn query_controller(
    conn: &Connection,
    query: &str,
    params: Option<Value>,
) -> Result<String, HttpError> {
    let (mut stmt, params) = match prepare_statement(conn, query, params) {
        Ok(v) => Ok(v),
        Err(e) => Err(bad_request(e.to_string())),
    }?;

    if is_select(query) {
        handle_select(stmt, params)
    } else if query.starts_with("INSERT") {
        handle_insert(&mut stmt, params)
    } else {
        handle_execute(&mut stmt, params)
    }
}

fn prepare_statement<'a>(
    conn: &'a Connection,
    query: &str,
    params: Option<Value>,
) -> Result<(Statement<'a>, ParamsFromIter<Vec<rusqlite::types::Value>>)> {
    match params {
        Some(params) if !params.is_array() => {
            let (bound_params, modified_query) = bind_object_to_params(params, query.to_string())?;
            let stmt = conn.prepare(&modified_query)?;
            Ok((stmt, bound_params))
        }
        Some(params) => {
            let bound_params = bind_array_to_params(params);
            let stmt = conn.prepare(query)?;
            Ok((stmt, bound_params))
        }
        None => {
            let stmt = conn.prepare(query)?;
            let empty_params = rusqlite::params_from_iter(Vec::<rusqlite::types::Value>::new());
            Ok((stmt, empty_params))
        }
    }
}

fn is_select(query: &str) -> bool {
    regex::Regex::new(r"^(?i)SELECT|^(?i)WITH RECURSIVE.*AS \(([\s\S]+?)\)\s*SELECT")
        .unwrap()
        .is_match(query)
}

#[instrument(err(Debug), skip(stmt, params))]
fn handle_select(
    stmt: Statement,
    params: ParamsFromIter<Vec<rusqlite::types::Value>>,
) -> Result<String, HttpError> {
    statement_to_vec(stmt, params)
        .map(|v| json!({ "data": v }).to_string())
        .map_err(|e| bad_request(e.to_string()))
}

#[instrument(err(Debug), skip(stmt, params))]
fn handle_insert(
    stmt: &mut Statement,
    params: ParamsFromIter<Vec<rusqlite::types::Value>>,
) -> Result<String, HttpError> {
    match stmt.insert(params) {
        Ok(rowid) => Ok(json!({ "data": [{ "success": true, "rowid": rowid }] }).to_string()),
        Err(e) => {
            if let Error::StatementChangedRows(0) = e {
                Ok(json!({ "data": [{ "success": false, "rowid": 0 }] }).to_string())
            } else {
                Err(bad_request(e.to_string()))
            }
        }
    }
}

#[instrument(err(Debug), skip(stmt, params))]
fn handle_execute(
    stmt: &mut Statement,
    params: ParamsFromIter<Vec<rusqlite::types::Value>>,
) -> Result<String, HttpError> {
    stmt.execute(params)
        .map(|changes| json!({ "data": [{ "success": true, "changes": changes }] }).to_string())
        .map_err(|e| bad_request(e.to_string()))
}

#[cfg(test)]
mod tests {
    use rusqlite::params;

    use super::*;

    #[test]
    fn test_query_controller() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE test (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            params![],
        )
        .unwrap();

        let query = "SELECT * FROM test";
        let params = None;
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[]}");

        let query = "INSERT INTO test (name) VALUES (?)";
        let params = Some(json!(["John Doe"]));
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");

        let query = "SELECT * FROM test";
        let params = None;
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"id\":1,\"name\":\"John Doe\"}]}");

        let query = "UPDATE test SET name = ? WHERE id = ?";
        let params = Some(json!(["Jane Doe", 1]));
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");

        let query = "SELECT * FROM test";
        let params = None;
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"id\":1,\"name\":\"Jane Doe\"}]}");

        let query = "DELETE FROM test WHERE id = ?";
        let params = Some(json!([1]));
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");

        let query = "SELECT * FROM test";
        let params = None;
        let result = query_controller(&conn, query, params).unwrap();
        assert_eq!(result, "{\"data\":[]}");
    }

    #[test]
    fn test_prepare_statement() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)",
            params![],
        )
        .unwrap();

        let query = "SELECT * FROM test";
        let params = None;
        let (stmt, bound_params) = prepare_statement(&conn, query, params).unwrap();
        assert_eq!(&stmt.expanded_sql().unwrap(), "SELECT * FROM test");
        assert_eq!(
            format!("{:?}", bound_params),
            String::from("ParamsFromIter([])"),
        );

        let query = "SELECT * FROM test WHERE id = :id AND name = :name";
        let params = Some(json!({ ":id": 1, ":name": "test" }));
        let (stmt, bound_params) = prepare_statement(&conn, query, params).unwrap();
        assert_eq!(
            &stmt.expanded_sql().unwrap(),
            "SELECT * FROM test WHERE id = NULL AND name = NULL"
        );
        assert_eq!(
            format!("{:?}", bound_params),
            String::from("ParamsFromIter([Integer(1), Text(\"test\")])")
        );

        let query = "SELECT * FROM test WHERE id = ? AND name = ?";
        let params = Some(json!([1, "test"]));
        let (stmt, bound_params) = prepare_statement(&conn, query, params).unwrap();
        assert_eq!(
            &stmt.expanded_sql().unwrap(),
            "SELECT * FROM test WHERE id = NULL AND name = NULL"
        );
        assert_eq!(
            format!("{:?}", bound_params),
            String::from("ParamsFromIter([Integer(1), Text(\"test\")])")
        );

        let query = "SELECT * FROM test WHERE id IN (:ids) AND name LIKE :pattern";
        let params = Some(json!({
            ":ids": [1, 2, 3],
            ":pattern": "%test%"
        }));
        let (stmt, bound_params) = prepare_statement(&conn, query, params).unwrap();
        assert_eq!(
            &stmt.expanded_sql().unwrap(),
            "SELECT * FROM test WHERE id IN (NULL) AND name LIKE NULL"
        );
        assert_eq!(
            format!("{:?}", bound_params),
            String::from("ParamsFromIter([Blob([1, 2, 3]), Text(\"%test%\")])")
        );
    }

    #[test]
    fn test_is_select() {
        let query = "SELECT * FROM test";
        assert!(is_select(query));

        let query = "WITH RECURSIVE cte AS (SELECT 1 AS n UNION ALL SELECT n + 1 FROM cte WHERE n < 10) SELECT * FROM cte";
        assert!(is_select(query));

        let query = "INSERT INTO test (name) VALUES (SELECT name FROM test)";
        assert!(!is_select(query));

        let query = "UPDATE test SET name = ? WHERE id = ?";
        assert!(!is_select(query));
    }

    #[test]
    fn test_handle_select() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE test (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            params![],
        )
        .unwrap();

        conn.execute("INSERT INTO test (name) VALUES (?)", params!["John Doe"])
            .unwrap();

        let query = "SELECT * FROM test";
        let params = rusqlite::params_from_iter(Vec::<rusqlite::types::Value>::new());
        let stmt = conn.prepare(query).unwrap();
        let result = handle_select(stmt, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"id\":1,\"name\":\"John Doe\"}]}");

        let query = "SELECT * FROM nonexistent_table";
        let stmt = conn.prepare(query);
        assert!(stmt.is_err());
    }

    #[test]
    fn test_handle_insert() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE test (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            params![],
        )
        .unwrap();

        let query = "INSERT INTO test (name) VALUES (?)";
        let params =
            rusqlite::params_from_iter(vec![rusqlite::types::Value::Text("John Doe".to_string())]);
        let mut stmt = conn.prepare(query).unwrap();
        let result = handle_insert(&mut stmt, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");

        let query = "INSERT INTO test (name) VALUES (?)";
        let params = rusqlite::params_from_iter(vec![rusqlite::types::Value::Text(
            "John Doe 2".to_string(),
        )]);
        let mut stmt = conn.prepare(query).unwrap();
        let result = handle_insert(&mut stmt, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":2,\"success\":true}]}");

        let query = "INSERT INTO nonexistent_table (name) VALUES (?)";
        let stmt = conn.prepare(query);
        assert!(stmt.is_err());
    }

    #[test]
    fn test_handle_execute() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE test (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            params![],
        )
        .unwrap();

        conn.execute("INSERT INTO test (name) VALUES (?)", params!["John Doe"])
            .unwrap();

        let query = "UPDATE test SET name = ? WHERE id = ?";
        let params = rusqlite::params_from_iter(vec![
            rusqlite::types::Value::Text("Jane Doe".to_string()),
            rusqlite::types::Value::Integer(1),
        ]);
        let mut stmt = conn.prepare(query).unwrap();
        let result = handle_execute(&mut stmt, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");

        conn.execute("INSERT INTO test (name) VALUES (?)", params!["John Doe"])
            .unwrap();

        let query = "UPDATE test SET name = ?";
        let params =
            rusqlite::params_from_iter(vec![rusqlite::types::Value::Text("Jane Doe".to_string())]);
        let mut stmt = conn.prepare(query).unwrap();
        let result = handle_execute(&mut stmt, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":2,\"success\":true}]}");

        let query = "UPDATE nonexistent_table SET name = ? WHERE id = ?";
        let stmt = conn.prepare(query);
        assert!(stmt.is_err());
    }
}
