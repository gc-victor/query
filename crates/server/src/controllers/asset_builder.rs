use anyhow::Result;
use hyper::{body::Incoming, Method, Request, Response};
use regex::Regex;
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
    sqlite::connect_db::connect_asset_db,
};

#[derive(Deserialize)]
struct AddAssetOptions {
    pub active: bool,
    pub data: ByteBuf,
    pub file_hash: String,
    pub mime_type: String,
    pub name: String,
}

#[derive(Deserialize)]
struct DeleteAssetOptions {
    pub name: String,
}

#[instrument(err(Debug), skip(req))]
pub async fn asset_builder(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match (req.method(), segments) {
        (&Method::DELETE, ["asset-builder"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: DeleteAssetOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match delete_asset(options) {
                Ok(_) => Ok(ok("")?),
                Err(e) => Err(e),
            }
        }
        (&Method::POST, ["asset-builder"]) => {
            // IMPORTANT! don't remove this validation
            validate_request(req)?;

            let body = Body::to_string(req.body_mut()).await?;

            let options: AddAssetOptions = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(bad_request(e.to_string())),
            }?;

            match add_asset(options) {
                Ok(_) => Ok(ok("")?),
                Err(e) => Err(e),
            }
        }
        _ => Err(not_implemented()),
    }
}

#[instrument(skip(options), fields(mime_type = options.mime_type, asset_name = options.name))]
fn add_asset(options: AddAssetOptions) -> Result<(), HttpError> {
    let AddAssetOptions {
        active,
        data,
        name,
        file_hash,
        mime_type,
    } = options;
    let re = Regex::new(r#"(\.[0-9a-z]+$)"#).unwrap();

    let connect = connect_asset_db()?;

    match connect.execute(
        "
        INSERT INTO asset
            (
                 active,
                 data,
                 name,
                 name_hashed,
                 mime_type
             )
         VALUES
            (
                :active,
                :data,
                :name,
                :name_hashed,
                :mime_type
            )
        ON CONFLICT(name) DO
        UPDATE SET
            active = :active,
            data = :data,
            name_hashed = :name_hashed,
            mime_type = :mime_type;
    ",
        named_params! {
            ":active": active,
            ":data": data.as_ref(),
            ":name": name,
            ":name_hashed": re.replace(&name, format!("-{}$1", file_hash)),
            ":mime_type": mime_type,
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(bad_request(e.to_string())),
    }
}

fn delete_asset(options: DeleteAssetOptions) -> Result<(), HttpError> {
    let connect = connect_asset_db()?;

    match connect.execute("DELETE FROM asset WHERE name = ?;", [options.name]) {
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
