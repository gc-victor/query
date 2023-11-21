use std::{fs, path::Path};

use hyper::{body::Incoming, Method, Request, Response};
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
    env::Env,
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

            let is_admin = is_admin(&token)?;

            // IMPORTANT! don't remove this validation
            if !is_admin && db_name == DB_CONFIG_NAME {
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

            match query_controller(&db_name, &query, params, is_admin) {
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

            let is_admin = is_admin(&token)?;

            // IMPORTANT! don't remove this validation
            if !is_admin && db_name == DB_CONFIG_NAME {
                return Err(bad_request(
                    "Can't query the config database without being admin".to_string(),
                ));
            }

            match query_controller(&db_name, &query, params, is_admin) {
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

fn query_controller(
    db_name: &str,
    query: &str,
    params: Option<Value>,
    is_admin: bool,
) -> Result<String, HttpError> {
    let conn = connect_db(db_name)?;

    if is_admin {
        let path = Env::dbs_path();

        if !Path::new(&path).exists() {
            fs::create_dir_all(&path).unwrap();
        }

        let db = format!("{}/{}", &path, db_name);

        conn.execute(&format!("ATTACH DATABASE ? AS {}", "_config"), [db])?;
    }

    if is_select(query) {
        if params.is_none() {
            let stmt = match conn.prepare(query) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }?;

            match statement_to_vec(stmt, []) {
                Ok(v) => Ok(json!({ "data": v }).to_string()),
                Err(e) => Err(e)?,
            }
        } else {
            let params = match params {
                Some(v) => Ok(v),
                None => Err(bad_request("Missing parameters".to_string())),
            }?;

            if params.is_array() {
                let params = bind_array_to_params(params);

                let stmt = match conn.prepare(query) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                }?;

                match statement_to_vec(stmt, params) {
                    Ok(v) => Ok(json!({ "data": v }).to_string()),
                    Err(e) => Err(e)?,
                }
            } else {
                let (params, query) = bind_object_to_params(params, query.to_string())?;

                let stmt = match conn.prepare(&query) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                }?;

                match statement_to_vec(stmt, params) {
                    Ok(v) => Ok(json!({ "data": v }).to_string()),
                    Err(e) => Err(e)?,
                }
            }
        }
    } else {
        #[warn(clippy::collapsible_else_if)]
        if params.is_none() {
            match conn.execute(query, []) {
                Ok(_) => Ok(json!({ "data": [{ "success": true }] }).to_string()),
                Err(e) => Err(bad_request(e.to_string())),
            }
        } else {
            let params = match params {
                Some(v) => Ok(v),
                None => Err(bad_request("Missing parameters".to_string())),
            }?;

            if !params.is_array() {
                let (params, query) = bind_object_to_params(params, query.to_string())?;

                match conn.execute(&query, params) {
                    Ok(_) => Ok(json!({ "data": [{ "success": true }] }).to_string()),
                    Err(e) => Err(bad_request(e.to_string())),
                }
            } else {
                let params = bind_array_to_params(params);

                match conn.execute(query, params) {
                    Ok(_) => Ok(json!({ "data": [{ "success": true }] }).to_string()),
                    Err(e) => Err(bad_request(e.to_string())),
                }
            }
        }
    }
}

fn is_select(query: &str) -> bool {
    regex::Regex::new(r"^(?i)SELECT|^(?i)WITH RECURSIVE.*AS \(([\s\S]+?)\)\s*SELECT")
        .unwrap()
        .is_match(query)
}
