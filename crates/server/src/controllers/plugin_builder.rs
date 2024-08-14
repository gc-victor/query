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
    sqlite::connect_db::connect_plugin_db,
};

#[derive(Deserialize)]
struct AddPluginOptions {
    pub data: ByteBuf,
    pub name: String,
    pub sha256: Option<String>,
}

#[derive(Deserialize)]
struct DeletePluginOptions {
    pub name: String,
}

#[instrument(err(Debug), skip(req))]
pub async fn plugin_builder(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match (req.method(), segments) {
        (&Method::DELETE, ["plugin-builder"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: DeletePluginOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match delete_plugin(options) {
                Ok(_) => Ok(ok("")?),
                Err(e) => Err(e),
            }
        }
        (&Method::POST, ["plugin-builder"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: AddPluginOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match add_plugin(options) {
                Ok(_) => Ok(ok("")?),
                Err(e) => Err(e),
            }
        }
        _ => Err(not_implemented()),
    }
}

#[instrument(skip(options), fields(name = options.name, sha256 = options.sha256))]
fn add_plugin(options: AddPluginOptions) -> Result<(), HttpError> {
    let AddPluginOptions { data, name, sha256 } = options;

    if !name.ends_with(".wasm") {
        return Err(bad_request(
            "The plugin name should end with .wasm".to_string(),
        ));
    }

    let connect = connect_plugin_db()?;

    match connect.execute(
        "
        INSERT INTO plugin
            (
                data,
                name,
                sha256
            )
         VALUES
            (
                :data,
                :name,
                :sha256
            )
        ON CONFLICT(name) DO
        UPDATE SET
            data = :data,
            sha256 = :sha256;
    ",
        named_params! {
            ":data": data.as_ref(),
            ":name": name,
            ":sha256": sha256,
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(bad_request(e.to_string())),
    }
}

fn delete_plugin(options: DeletePluginOptions) -> Result<(), HttpError> {
    let connect = connect_plugin_db()?;

    match connect.execute("DELETE FROM plugin WHERE name = ?;", [options.name]) {
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
