use anyhow::Result;
use hyper::{body::Incoming, Method, Request, Response};
use rusqlite::named_params;
use serde::Deserialize;
use serde_bytes::ByteBuf;
use tracing::instrument;

use crate::{
    controllers::utils::{
        body::{Body, BoxBody},
        get_token::get_token,
        http_error::{bad_request, not_implemented, HttpError},
        responses::ok,
        validate_is_admin::validate_is_admin,
        validate_token::validate_token,
        validate_token_creation::validate_token_creation,
    },
    sqlite::connect_db::connect_function_db,
};

#[derive(Deserialize)]
struct AddFunctionOptions<'a> {
    pub function: ByteBuf,
    pub method: &'a str,
    pub path: &'a str,
}

#[derive(Deserialize)]
struct DeleteFunctionOptions<'a> {
    pub method: &'a str,
    pub path: &'a str,
}

#[instrument(err(Debug), skip(req))]
pub async fn function_builder(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match (req.method(), segments) {
        (&Method::DELETE, ["function-builder"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: DeleteFunctionOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match delete_function(options) {
                Ok(_) => Ok(ok("")?),
                Err(e) => Err(e),
            }
        }
        (&Method::POST, ["function-builder"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: AddFunctionOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match add_function(options) {
                Ok(_) => Ok(ok("")?),
                Err(e) => Err(e),
            }
        }
        _ => Err(not_implemented()),
    }
}

fn add_function(options: AddFunctionOptions) -> Result<(), HttpError> {
    let connect = connect_function_db()?;

    match connect.execute(
        "
        INSERT INTO function
            (
                active,
                method,
                path,
                function
            )
        VALUES
            (
                :active,
                :method,
                :path,
                :function
            )
        ON CONFLICT(method, path) DO
        UPDATE SET function = :function;
    ",
        named_params! {
            ":active": 1,
            ":method": options.method,
            ":path": options.path,
            ":function": options.function.as_ref(),
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(bad_request(e.to_string())),
    }
}

fn delete_function(options: DeleteFunctionOptions) -> Result<(), HttpError> {
    let connect = connect_function_db()?;

    match connect.execute(
        "
        DELETE FROM
            function
        WHERE
            method = :method
        AND
            path = :path;
    ",
        named_params! {
            ":method": options.method,
            ":path": options.path,
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(bad_request(e.to_string())),
    }
}

fn validate_request(req: &Request<Incoming>) -> Result<(), HttpError> {
    // IMPORTANT! don't remove this validation
    validate_token_creation()?;

    let token = get_token(req.headers().to_owned())?;

    // IMPORTANT! don't remove this validation
    validate_token(&token)?;
    // IMPORTANT! don't remove this validation
    validate_is_admin(&token)?;

    Ok(())
}
