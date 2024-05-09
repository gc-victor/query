use anyhow::{anyhow, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chacha20poly1305::aead::OsRng;
use hyper::{body::Incoming, Method, Request, Response};
use rusqlite::named_params;
use serde::Deserialize;
use serde_json::json;
use tracing::instrument;

use crate::{
    controllers::utils::{
        get_token::get_token,
        http_error::{bad_request, internal_server_error, not_found, HttpError},
        responses::{created, ok},
        statement_to_vec::statement_to_vec,
        validate_is_admin::validate_is_admin,
        validate_token::validate_token,
        validate_user_email::validate_user_email,
    },
    sqlite::connect_db::connect_config_db,
};

use super::utils::body::{Body, BoxBody};

#[derive(Deserialize)]
struct CreateUserOptions {
    email: String,
    password: String,
    admin: Option<bool>,
    active: Option<bool>,
}

#[derive(Deserialize)]
struct DeleteUserOptions {
    email: String,
}

#[derive(Deserialize)]
struct UpdateUserOptions {
    email: String,
    new_email: Option<String>,
    new_password: Option<String>,
    admin: Option<bool>,
    active: Option<bool>,
}

#[instrument(err(Debug), skip(req))]
pub async fn user(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match (req.method(), segments) {
        (&Method::GET, ["user"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            match list_users() {
                Ok(u) => match ok(u) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::POST, ["user"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: CreateUserOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match create_user(options) {
                Ok(_) => match created() {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::DELETE, ["user"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let DeleteUserOptions { email } = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            // IMPORTANT! don't remove this validation
            validate_user_email(&email)?;

            match delete_user(&email) {
                Ok(_) => match ok("") {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        (&Method::PUT, ["user"]) => {
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: UpdateUserOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match update_user(options) {
                Ok(_) => match ok("".to_string()) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(internal_server_error(e.to_string())),
            }
        }
        _ => Err(not_found()),
    }
}

fn list_users() -> Result<String> {
    let conn = connect_config_db()?;

    let stmt = match conn.prepare("SELECT * FROM _config_user") {
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

fn create_user(options: CreateUserOptions) -> Result<()> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"
        INSERT OR IGNORE INTO
            _config_user(
                email,
                password,
                admin,
                active
            )
        VALUES (
            :email,
            :password,
            :active,
            :admin
        )
        "#,
        named_params! {
            ":email": options.email,
            ":password": hash_password(&options.password),
            ":active": options.active.unwrap_or(true),
            ":admin": options.admin.unwrap_or(true),
        },
    )?;

    Ok(())
}

fn delete_user(email: &str) -> Result<()> {
    let conn = connect_config_db()?;

    conn.execute(r#"DELETE FROM _config_user WHERE email = ?"#, [email])?;

    Ok(())
}

fn update_user(options: UpdateUserOptions) -> Result<()> {
    let conn = connect_config_db()?;

    conn.execute(
        r#"
        UPDATE
            _config_user
        SET
            email = :new_email,
            password = :new_password,
            active = :active,
            admin = :admin
        WHERE
            email = :email
        "#,
        named_params! {
            ":email": options.email,
            ":new_email": match options.new_email {
                Some(e) => e,
                None => options.email.to_string()
            },
            ":new_password": match options.new_password {
                Some(p) => hash_password(&p),
                None => conn.query_row("SELECT password FROM _config_user WHERE email = ?", [&options.email], |row| row.get(0))?,
            },
            ":active": match options.active {
                Some(a) => a,
                None => conn.query_row("SELECT active FROM _config_user WHERE email = ?", [&options.email], |row| row.get(0))?,
            },
            ":admin": match options.admin {
                Some(a) => a,
                None => conn.query_row("SELECT admin FROM _config_user WHERE email = ?", [&options.email], |row| row.get(0))?,
            },
        },
    )?;

    Ok(())
}

fn validate_request(req: &Request<Incoming>) -> Result<(), HttpError> {
    // IMPORTANT! don't remove this validation
    validate_user_creation()?;

    let token = get_token(req.headers().to_owned())?;

    // IMPORTANT! don't remove this validation
    validate_token(&token)?;
    // IMPORTANT! don't remove this validation
    validate_is_admin(&token)?;

    Ok(())
}

fn validate_user_creation() -> Result<(), HttpError> {
    // NOTE: configure to allow or not to create users and projects
    match connect_config_db()?.query_row(
        "SELECT value FROM _config_option WHERE name = 'create_user'",
        [],
        |row| -> std::result::Result<String, rusqlite::Error> { row.get(0) },
    ) {
        Ok(s) if s == "1" => Ok(()),
        Ok(s) if s == "0" => Err(not_found()),
        _ => Err(internal_server_error(
            "Error getting the value of the option 'create_token'".to_string(),
        )),
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}
