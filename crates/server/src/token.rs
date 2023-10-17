use anyhow::{anyhow, Result};
use hyper::{Body, Method, Request, Response};
use rusqlite::named_params;
use serde::Deserialize;
use serde_json::json;
use tracing::instrument;

use crate::{
    sqlite::connect_db::connect_config_db,
    utils::{
        current_time::current_time_millis,
        get_body::get_body,
        get_query_string::get_query_string,
        get_token::get_token,
        http_error::{bad_request, internal_server_error, not_found, HttpError},
        responses::{created, ok},
        statement_to_vec::statement_to_vec,
        validate_is_admin::validate_is_admin,
        validate_token::validate_token,
        validate_token_creation::validate_token_creation,
    },
};

#[derive(Deserialize)]
struct CreateTokenOptions {
    name: String,
    expiration_date: Option<i64>,
    active: Option<bool>,
    write: Option<bool>,
}

#[derive(Deserialize)]
struct DeleteTokenOptions {
    name: String,
}

#[derive(Deserialize)]
struct UpdateTokenOptions {
    name: String,
    expiration_date: Option<i64>,
    active: Option<bool>,
    write: Option<bool>,
}

#[instrument(err(Debug), skip(req))]
pub async fn token(
    req: &mut Request<Body>,
    segments: &[&str],
) -> Result<Response<Body>, HttpError> {
    match (req.method(), segments) {
        (&Method::GET, ["token"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            match list_tokens() {
                Ok(s) => match ok(s) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::POST, ["token"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = get_body(req).await?;

            let options: CreateTokenOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match crate_token(options) {
                Ok(_) => match created() {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        (&Method::GET, ["token", "value"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let name = match get_query_string(req, "name") {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match get_token_value(&name) {
                Ok(t) => match ok(t) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::DELETE, ["token"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = get_body(req).await?;

            let DeleteTokenOptions { name } = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match delete_token(&name) {
                Ok(_) => match ok("") {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        (&Method::PUT, ["token"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = get_body(req).await?;

            let options: UpdateTokenOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match update_token(options) {
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
    // IMPORTANT! don't remove this validation
    validate_token_creation()?;

    let token = get_token(req)?;

    // IMPORTANT! don't remove this validation
    validate_token(&token)?;
    // IMPORTANT! don't remove this validation
    validate_is_admin(&token)?;

    Ok(())
}

fn crate_token(options: CreateTokenOptions) -> Result<(), HttpError> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"
        INSERT OR IGNORE INTO
            _config_token(
                name,
                token,
                expiration_date,
                active,
                write
            )
        VALUES (
            :name,
            token('{
                "sub": "' || (SELECT uuid()) ||  '",
                "exp": ' || :expiration_date || ',
                "iat": ' || strftime('%s', datetime('now')) || ',
                "iss": "token"
            }'),
            :expiration_date,
            :active,
            :write
        )
        "#,
        named_params! {
            ":name": options.name,
            ":expiration_date": match options.expiration_date {
                Some(e) => {
                    if e < current_time_millis() {
                        return Err(bad_request("The expiration_date must be greater than current time".to_string()))
                    } else {
                        e
                    }
                },
                None => current_time_millis(),
            },
            ":active": options.active.unwrap_or(true),
            ":write": options.write.unwrap_or(true),
        },
    )?;

    Ok(())
}

fn get_token_value(name: &str) -> Result<String> {
    let conn = connect_config_db()?;

    match conn.query_row(
        "
        SELECT
            token
        FROM
            _config_token
        WHERE
            name = ?
        ",
        [name],
        |row| -> std::result::Result<String, rusqlite::Error> { row.get(0) },
    ) {
        Ok(v) => Ok(json!({ "data": [{ "token": v }] }).to_string()),
        Err(e) => {
            if let rusqlite::Error::QueryReturnedNoRows = e {
                Ok(json!({ "data": [] }).to_string())
            } else {
                Err(anyhow!(e))
            }
        }
    }
}

fn delete_token(name: &str) -> Result<(), HttpError> {
    let conn = connect_config_db()?;

    conn.execute(r#"DELETE FROM _config_token WHERE name = ?"#, [name])?;

    Ok(())
}

fn list_tokens() -> Result<String> {
    let conn = connect_config_db()?;

    let stmt = match conn.prepare("SELECT * FROM _config_token") {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }?;

    match statement_to_vec(stmt, []) {
        Ok(v) => Ok(json!({ "data": v }).to_string()),
        Err(e) => {
            if let rusqlite::Error::QueryReturnedNoRows = e {
                Ok(json!({ "data": [] }).to_string())
            } else {
                Err(anyhow!(e))
            }
        }
    }
}

fn update_token(options: UpdateTokenOptions) -> Result<(), HttpError> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"
        UPDATE
            _config_token
        SET
            active = :active,
            expiration_date = :expiration_date,
            token = token('{
                "sub": "' || (SELECT uuid()) ||  '", 
                "exp": ' || :expiration_date || ',
                "iat": ' || strftime('%s', datetime('now')) || ',
                "iss": "token"
            }'),
            write = :write
        WHERE
            name = :name;
        "#,
        named_params! {
            ":name": options.name,
            ":expiration_date": match options.expiration_date {
                Some(e) => e,
                None => conn.query_row("SELECT expiration_date FROM _config_token WHERE name = ?", [&options.name], |row| row.get(0))?,
            },
            ":active": match options.active {
                Some(a) => a,
                None => conn.query_row("SELECT active FROM _config_token WHERE name = ?", [&options.name], |row| row.get(0))?,
            },
            ":write": match options.write {
                Some(w) => w,
                None => conn.query_row("SELECT write FROM _config_token WHERE name = ?", [&options.name], |row| row.get(0))?,
            },
        },
    )?;

    Ok(())
}
