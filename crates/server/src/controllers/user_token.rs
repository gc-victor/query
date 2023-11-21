use anyhow::{anyhow, Result};
use hyper::{body::Incoming, Method, Request, Response};
use rusqlite::named_params;
use serde::Deserialize;
use serde_json::json;
use tracing::instrument;

use crate::{
    controllers::utils::{
        body::{Body, BoxBody},
        current_time::current_time_millis,
        get_query_string::get_query_string,
        get_token::get_token,
        http_error::{bad_request, internal_server_error, not_found, HttpError},
        responses::{created, ok},
        statement_to_vec::statement_to_vec,
        validate_is_admin::validate_is_admin,
        validate_token::validate_token,
        validate_token_creation::validate_token_creation,
        validate_user_creation::validate_user_creation,
        validate_user_email::validate_user_email,
        validate_user_password::validate_user_password,
    },
    sqlite::connect_db::connect_config_db,
};

#[derive(Deserialize)]
struct CreateUserTokenOptions {
    email: String,
    expiration_date: Option<i64>,
    write: Option<bool>,
}

#[derive(Deserialize)]
struct GetUserTokenValueWithoutTokenOptions {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct DeleteUserTokenOptions {
    email: String,
}

#[derive(Deserialize)]
struct UpdateUserTokenOptions {
    email: String,
    expiration_date: Option<i64>,
    write: Option<bool>,
}

#[instrument(err(Debug), skip(req))]
pub async fn user_token(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match (req.method(), segments) {
        (&Method::GET, ["user", "token"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            match list_users_tokens() {
                Ok(u) => match ok(u) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::POST, ["user", "token"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: CreateUserTokenOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            // IMPORTANT! don't remove this validation
            validate_emails_user_exists(&options.email)?;

            match create_user_token(options) {
                Ok(_) => match created() {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        (&Method::GET, ["user", "token", "value"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let email = match get_query_string(req, "email") {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match get_user_token_value(&email) {
                Ok(t) => match ok(t) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::POST, ["user", "token", "value"]) => {
            // IMPORTANT! don't remove this validation
            validate_user_creation()?;

            // IMPORTANT! don't remove this validation
            validate_token_creation()?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: GetUserTokenValueWithoutTokenOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            // IMPORTANT! don't remove this validation
            validate_user_password(&options.email, &options.password)?;

            match get_user_token_value(&options.email) {
                Ok(t) => match ok(t) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::DELETE, ["user", "token"]) => {
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let DeleteUserTokenOptions { email } = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            // TODO: test it
            // IMPORTANT! don't remove this validation
            validate_user_email(&email)?;

            match delete_user_token(&email) {
                Ok(_) => match ok("") {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::PUT, ["user", "token"]) => {
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: UpdateUserTokenOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match update_user_token(options) {
                Ok(_) => match ok("".to_string()) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        _ => Err(not_found()),
    }
}

fn list_users_tokens() -> Result<String> {
    let conn = connect_config_db()?;

    let stmt = match conn.prepare("SELECT * FROM _config_user_token") {
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

fn get_user_token_value(email: &str) -> Result<String> {
    let conn = connect_config_db()?;

    match conn.query_row(
        "
        SELECT
            token
        FROM
            _config_user_token
        WHERE
            user_uuid = (SELECT uuid FROM _config_user WHERE email = ?)
        ",
        [email],
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

fn create_user_token(options: CreateUserTokenOptions) -> Result<(), HttpError> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"
        INSERT OR IGNORE INTO
            _config_user_token(
                user_uuid,
                token,
                expiration_date,
                write
            )
        VALUES
            (
                (SELECT uuid FROM _config_user WHERE email = :email),
                token('{"sub": "' || (SELECT uuid()) ||  '", "exp": ' || strftime('%s', datetime('now')) || ', "iat": ' || strftime('%s', datetime('now')) || ', "iss": "user_token"}'),
                :expiration_date,
                :write
            )
        "#,
        named_params! {
            ":email": options.email,
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
            ":write": options.write.unwrap_or(true)
        },
    )?;

    Ok(())
}

fn delete_user_token(email: &str) -> Result<()> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"DELETE FROM _config_user_token WHERE user_uuid = (SELECT uuid FROM _config_user WHERE email = :email)"#,
        [email],
    )?;

    Ok(())
}

fn update_user_token(options: UpdateUserTokenOptions) -> Result<(), HttpError> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"
        UPDATE
            _config_user_token
        SET
            expiration_date = :expiration_date,
            write = :write
        WHERE
            user_uuid = (SELECT uuid FROM _config_user WHERE email = :email)
        "#,
        named_params! {
            ":email": options.email,
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
            ":write": options.write.unwrap_or(true)
        },
    )?;

    Ok(())
}

fn validate_request(req: &Request<Incoming>) -> Result<(), HttpError> {
    // IMPORTANT! don't remove this validation
    validate_user_creation()?;

    // IMPORTANT! don't remove this validation
    validate_token_creation()?;

    let token = get_token(req.headers().to_owned())?;

    // IMPORTANT! don't remove this validation
    validate_token(&token)?;
    // IMPORTANT! don't remove this validation
    validate_is_admin(&token)?;

    Ok(())
}

fn validate_emails_user_exists(email: &str) -> Result<(), HttpError> {
    let conn = connect_config_db()?;

    match conn.query_row(
        "SELECT COUNT(*) FROM _config_user WHERE email = :email",
        [email],
        |row| -> std::result::Result<u64, rusqlite::Error> { row.get(0) },
    ) {
        Ok(0) => Err(bad_request("There user do not exists".to_string())),
        _ => Ok(()),
    }?;

    Ok(())
}
