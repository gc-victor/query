use anyhow::Result;
use hyper::{Body, Method, Request, Response};
use serde::Deserialize;
use tracing::instrument;

use crate::{
    sqlite::connect_db::connect_db,
    utils::{
        get_body::get_body,
        get_token::get_token,
        http_error::{bad_request, internal_server_error, not_found, unauthorized, HttpError},
        responses::created,
        validate_is_admin::validate_is_admin,
        validate_token::validate_token,
        validate_user_creation::validate_user_creation,
        validate_write::validate_write,
    },
};

#[derive(Deserialize)]
struct MigrationOptions {
    pub db_name: String,
    pub query: String,
}

#[instrument(err(Debug), skip(req))]
pub async fn migration(
    req: &mut Request<Body>,
    segments: &[&str],
) -> Result<Response<Body>, HttpError> {
    match (req.method(), segments) {
        (&Method::POST, ["migration"]) => {
            // IMPORTANT! don't remove this validation
            validate_user_creation()?;

            let token = get_token(req)?;

            // IMPORTANT! don't remove this validation
            validate_is_admin(&token)?;

            // IMPORTANT! don't remove this validation
            validate_token(&token)?;

            // IMPORTANT! don't remove this validation
            if !validate_write(&token)? {
                tracing::error!("Token without write permission tried to write to the database");
                return Err(unauthorized());
            }

            let body = get_body(req).await?;

            let MigrationOptions { db_name, query } = match serde_json::from_str(&body) {
                Ok(v) => Ok(v),
                Err(e) => Err(internal_server_error(e.to_string())),
            }?;

            match migration_controller(&db_name, &query) {
                Ok(_) => match created() {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                },
                Err(e) => Err(e),
            }
        }
        _ => Err(not_found()),
    }
}

fn migration_controller(db_name: &str, query: &str) -> Result<(), HttpError> {
    let conn = connect_db(db_name)?;

    match conn.execute_batch(
        format!(
            r#"
                BEGIN;
                {}
                COMMIT;
            "#,
            &query
        )
        .as_str(),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(bad_request(e.to_string())),
    }
}
