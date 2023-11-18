use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use hyper::{Body, Method, Request, Response};
use rusqlite::limits::Limit;
use serde::Deserialize;
use serde_json::json;
use tracing::instrument;

use crate::{
    controllers::utils::{
        get_body::get_body,
        get_token::get_token,
        http_error::{bad_request, internal_server_error, not_found, HttpError},
        responses::{created, ok},
        validate_is_admin::validate_is_admin,
        validate_token::validate_token,
    },
    env::Env,
    sqlite::connect_db::connect_db,
};

#[derive(Deserialize)]
struct CreateBranchOptions {
    pub db_name: String,
    pub branch_name: String,
}

#[derive(Deserialize)]
struct DeleteBranchOptions {
    pub db_name: String, // has to have branch in the name
}

#[instrument(err(Debug), skip(req))]
pub async fn branch(
    req: &mut Request<Body>,
    segments: &[&str],
) -> Result<Response<Body>, HttpError> {
    match (req.method(), segments) {
        (&Method::GET, ["branch"]) => {
            validate_request(req)?;

            match list_branches() {
                Ok(u) => match ok(u) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        (&Method::POST, ["branch"]) => {
            validate_request(req)?;

            let body = get_body(req).await?;

            let options: CreateBranchOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match create_branch(options) {
                Ok(_) => match created() {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        (&Method::DELETE, ["branch"]) => {
            validate_request(req)?;

            let body = get_body(req).await?;

            let options: DeleteBranchOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match delete_branch(options) {
                Ok(_) => match ok("") {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        _ => Err(not_found()),
    }
}

fn validate_request(req: &Request<Body>) -> Result<(), HttpError> {
    let token = get_token(req)?;

    // IMPORTANT! don't remove this validation
    validate_token(&token)?;
    // IMPORTANT! don't remove this validation
    validate_is_admin(&token)?;

    Ok(())
}

fn list_branches() -> Result<String, HttpError> {
    let dir = &Env::dbs_path();

    let mut values = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(v) => Ok(v),
        Err(e) => Err(internal_server_error(e.to_string())),
    }?;

    for entry in entries {
        let entry = match entry {
            Ok(v) => Ok(v),
            Err(e) => Err(internal_server_error(e.to_string())),
        }?;

        let path = entry.path();

        let mut hash_map = HashMap::new();

        if path.is_file() && path.to_string_lossy().ends_with(".branch.sql") {
            hash_map.insert(
                "branch",
                path.display().to_string().replace(&format!("{}/", dir), ""),
            );
        }

        if !hash_map.is_empty() {
            values.push(hash_map);
        }
    }

    Ok(json!({ "data": values }).to_string())
}

fn create_branch(options: CreateBranchOptions) -> Result<(), HttpError> {
    let branch_name = &options.branch_name;
    let db_name = &options.db_name;

    let branch_name = &format!(
        "{}.{}.branch.sql",
        db_name.trim_end_matches(".sql").trim_end_matches(".db"),
        branch_name
    );
    let branch_name_path = &format!("{}/{}", Env::dbs_path(), branch_name);

    if Path::new(branch_name_path).exists() {
        return Err(bad_request(format!(
            "The branch {} already exists",
            branch_name
        )));
    }

    let conn = connect_db(db_name)?;

    conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 1);

    match conn.execute("VACUUM INTO ?;", [branch_name_path]) {
        Ok(_) => Ok(()),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

fn delete_branch(options: DeleteBranchOptions) -> Result<(), HttpError> {
    let db_name = options.db_name;

    if !db_name.ends_with(".branch.sql") {
        return Err(bad_request(format!(
            r#"The database name "{}" doesn't corresponds with a branch name"#,
            db_name
        )));
    }

    let path = &format!("{}/{}", Env::dbs_path(), db_name);

    if !Path::new(path).exists() {
        return Err(bad_request(format!(
            "The database {} doesn't exist",
            db_name
        )));
    }

    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}
