use hyper::{body::Incoming, Method, Request, Response};
use rusqlite::{Connection, Error, Statement};
use serde::Deserialize;

use anyhow::Result;
use serde_json::{json, Value};
use tracing::instrument;

use crate::{
    constants::DB_CONFIG_NAME,
    controllers::utils::{
        bind_to_params::{bind_array_to_params, bind_named_params},
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
    let mut stmt = conn.prepare(query)?;
    let empty_params = rusqlite::params_from_iter(Vec::<rusqlite::types::Value>::new());

    if is_select(query) {
        match params {
            Some(params) if !params.is_array() => {
                let params_bound = bind_named_params(params);
                let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
                    .iter()
                    .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
                    .collect::<Vec<_>>();
                handle_select(stmt, params)
            }
            Some(params) => handle_select(stmt, bind_array_to_params(params)),
            None => {
                let stmt = conn.prepare(query)?;
                handle_select(stmt, empty_params)
            }
        }
    } else if query.starts_with("INSERT") {
        match params {
            Some(params) if !params.is_array() => {
                let params_bound = bind_named_params(params);
                let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
                    .iter()
                    .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
                    .collect::<Vec<_>>();
                handle_insert(stmt, params)
            }
            Some(params) => handle_insert(stmt, bind_array_to_params(params)),
            None => handle_insert(stmt, empty_params),
        }
    } else {
        match params {
            Some(params) if !params.is_array() => {
                let params_bound = bind_named_params(params);
                let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
                    .iter()
                    .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
                    .collect::<Vec<_>>();
                handle_execute(&mut stmt, params)
            }
            Some(params) => {
                handle_execute(&mut stmt, bind_array_to_params(params))
            }
            None => handle_execute(&mut stmt, empty_params),
        }
    }
}

fn is_select(query: &str) -> bool {
    regex::Regex::new(r"^(?i)SELECT|^(?i)WITH RECURSIVE.*AS \(([\s\S]+?)\)\s*SELECT")
        .unwrap()
        .is_match(query)
}

#[instrument(err(Debug), skip(stmt, params))]
fn handle_select<P>(stmt: Statement, params: P) -> Result<String, HttpError>
where
    P: rusqlite::Params,
{
    statement_to_vec(stmt, params)
        .map(|v| json!({ "data": v }).to_string())
        .map_err(|e| bad_request(e.to_string()))
}

#[instrument(err(Debug), skip(stmt, params))]
fn handle_insert<P>(mut stmt: Statement, params: P) -> Result<String, HttpError>
where
    P: rusqlite::Params + Clone,
{
    match stmt.insert(params.clone()) {
        Ok(rowid) => Ok(json!({ "data": [{ "success": true, "rowid": rowid }] }).to_string()),
        Err(Error::StatementChangedRows(0)) => {
            Ok(json!({ "data": [{ "success": false, "rowid": 0 }] }).to_string())
        }
        Err(_) => stmt
            .execute(params)
            .map(|changes| json!({ "data": [{ "success": true, "changes": changes }] }).to_string())
            .map_err(|e| bad_request(e.to_string())),
    }
}

#[instrument(err(Debug), skip(stmt, params))]
fn handle_execute<P>(stmt: &mut Statement, params: P) -> Result<String, HttpError>
where
    P: rusqlite::Params,
{
    stmt.execute(params)
        .map(|changes| json!({ "data": [{ "success": true, "changes": changes }] }).to_string())
        .map_err(|e| bad_request(e.to_string()))
}

#[cfg(test)]
mod tests {
    use rusqlite::params;

    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE test (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                age INTEGER,
                email TEXT
            )",
            params![],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_empty_select() {
        let conn = setup_test_db();
        let result = query_controller(&conn, "SELECT * FROM test", None).unwrap();
        assert_eq!(result, "{\"data\":[]}");
    }

    #[test]
    fn test_basic_insert_with_positional_params() {
        let conn = setup_test_db();
        let result = query_controller(
            &conn,
            "INSERT INTO test (name, age) VALUES (?, ?)",
            Some(json!(["John Doe", 30])),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");

        let select_result = query_controller(&conn, "SELECT * FROM test", None).unwrap();
        assert_eq!(
            select_result,
            "{\"data\":[{\"age\":30,\"email\":null,\"id\":1,\"name\":\"John Doe\"}]}"
        );
    }

    #[test]
    fn test_colon_parameters() {
        let conn = setup_test_db();
        let result = query_controller(
            &conn,
            "INSERT INTO test (name, email) VALUES (:name, :email)",
            Some(json!({
                ":name": "Bob Smith",
                ":email": "bob@example.com"
            })),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");

        let select_result = query_controller(
            &conn,
            "SELECT * FROM test WHERE email = :email",
            Some(json!({
                ":email": "bob@example.com"
            })),
        )
        .unwrap();
        assert_eq!(
            select_result,
            "{\"data\":[{\"age\":null,\"email\":\"bob@example.com\",\"id\":1,\"name\":\"Bob Smith\"}]}"
        );
    }

    #[test]
    fn test_at_parameters() {
        let conn = setup_test_db();
        let result = query_controller(
            &conn,
            "INSERT INTO test (name, email) VALUES (@name, @email)",
            Some(json!({
                "@name": "Bob Smith",
                "@email": "bob@example.com"
            })),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");

        let select_result = query_controller(
            &conn,
            "SELECT * FROM test WHERE email = @email",
            Some(json!({
                "@email": "bob@example.com"
            })),
        )
        .unwrap();
        assert_eq!(
            select_result,
            "{\"data\":[{\"age\":null,\"email\":\"bob@example.com\",\"id\":1,\"name\":\"Bob Smith\"}]}"
        );
    }

    #[test]
    fn test_dolar_parameters() {
        let conn = setup_test_db();
        let result = query_controller(
            &conn,
            "INSERT INTO test (name, email) VALUES ($name, $email)",
            Some(json!({
                "$name": "Bob Smith",
                "$email": "bob@example.com"
            })),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");

        let select_result = query_controller(
            &conn,
            "SELECT * FROM test WHERE email = $email",
            Some(json!({
                "$email": "bob@example.com"
            })),
        )
        .unwrap();
        assert_eq!(
            select_result,
            "{\"data\":[{\"age\":null,\"email\":\"bob@example.com\",\"id\":1,\"name\":\"Bob Smith\"}]}"
        );
    }

    #[test]
    fn test_multiple_inserts_and_select() {
        let conn = setup_test_db();

        query_controller(
            &conn,
            "INSERT INTO test (name) VALUES (?), (?), (?)",
            Some(json!(["Alice", "Bob", "Charlie"])),
        )
        .unwrap();

        let result = query_controller(&conn, "SELECT * FROM test", None).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        assert!(result.contains("Charlie"));
    }

    #[test]
    fn test_update_named_parameters() {
        let conn = setup_test_db();

        query_controller(
            &conn,
            "INSERT INTO test (name, age) VALUES (?, ?)",
            Some(json!(["John Doe", 30])),
        )
        .unwrap();

        let result = query_controller(
            &conn,
            "UPDATE test SET age = :new_age WHERE name = :name",
            Some(json!({
                ":new_age": 31,
                ":name": "John Doe"
            })),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");
    }

    #[test]
    fn test_delete_operations() {
        let conn = setup_test_db();

        query_controller(
            &conn,
            "INSERT INTO test (name) VALUES (?)",
            Some(json!(["ToDelete"])),
        )
        .unwrap();

        let result = query_controller(
            &conn,
            "DELETE FROM test WHERE name = ?",
            Some(json!(["ToDelete"])),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");

        let select_result = query_controller(&conn, "SELECT * FROM test", None).unwrap();
        assert_eq!(select_result, "{\"data\":[]}");
    }

    #[test]
    #[should_panic(expected = "syntax error in INVALID SQL at offset 0")]
    fn test_invalid_sql() {
        let conn = setup_test_db();
        query_controller(&conn, "INVALID SQL", None).unwrap();
    }

    #[test]
    #[should_panic(expected = "Wrong number of parameters passed to query. Got 2, needed 1")]
    fn test_parameter_mismatch() {
        let conn = setup_test_db();
        query_controller(
            &conn,
            "INSERT INTO test (name) VALUES (?)",
            Some(json!(["name", "extra_param"])),
        )
        .unwrap();
    }

    #[test]
    fn test_null_handling() {
        let conn = setup_test_db();
        let result = query_controller(
            &conn,
            "INSERT INTO test (name, age, email) VALUES (?, ?, ?)",
            Some(json!(["Test", null, null])),
        )
        .unwrap();
        assert_eq!(result, "{\"data\":[{\"rowid\":1,\"success\":true}]}");
    }

    #[test]
    fn test_complex_query() {
        let conn = setup_test_db();

        query_controller(
            &conn,
            "INSERT INTO test (name, age) VALUES (?, ?), (?, ?)",
            Some(json!(["Young", 20, "Old", 60])),
        )
        .unwrap();

        let result = query_controller(
            &conn,
            "SELECT name, age FROM test WHERE age > :min_age AND age < :max_age",
            Some(json!({
                ":min_age": 18,
                ":max_age": 30
            })),
        )
        .unwrap();
        assert!(result.contains("Young"));
        assert!(!result.contains("Old"));
    }

    #[test]
    fn test_handle_execute_with_named_params() {
        let conn = setup_test_db();

        conn.execute("INSERT INTO test (name) VALUES (?)", params!["John Doe"])
            .unwrap();

        let query = "UPDATE test SET name = :new_name WHERE id = :id";
        let mut stmt = conn.prepare(query).unwrap();
        let named_params = vec![
            (":new_name", &"Jane Doe" as &dyn rusqlite::ToSql),
            (":id", &1 as &dyn rusqlite::ToSql),
        ];
        let result = handle_execute(&mut stmt, named_params.as_slice()).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");

        conn.execute("INSERT INTO test (name) VALUES (?)", params!["John Doe"])
            .unwrap();

        let query = "UPDATE test SET name = :new_name";
        let mut stmt = conn.prepare(query).unwrap();
        let named_params = vec![(":new_name", &"Jane Doe" as &dyn rusqlite::ToSql)];
        let result = handle_execute(&mut stmt, named_params.as_slice()).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":2,\"success\":true}]}");

        let query = "UPDATE nonexistent_table SET name = :new_name WHERE id = :id";
        let stmt = conn.prepare(query);
        assert!(stmt.is_err());
    }

    #[test]
    fn test_handle_execute_with_iter_params() {
        let conn = setup_test_db();

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

        let query = "UPDATE test SET name = ?";
        let params = rusqlite::params_from_iter(vec![rusqlite::types::Value::Text(
            "John Smith".to_string(),
        )]);
        let mut stmt = conn.prepare(query).unwrap();
        let result = handle_execute(&mut stmt, params).unwrap();
        assert_eq!(result, "{\"data\":[{\"changes\":1,\"success\":true}]}");
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
}
