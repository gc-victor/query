pub mod constants;

mod migration;
mod query;
pub mod sqlite;
mod token;
mod user;
pub mod user_token;
pub mod utils;

use std::convert::Infallible;
use std::net::SocketAddr;

use anyhow::Result;
use dotenv::dotenv;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use tracing::{subscriber::set_global_default, Instrument};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, Registry};

use crate::{
    sqlite::create_config_db::create_config_db,
    utils::env::Env,
    utils::http_error::HttpError,
    utils::responses::{
        bad_request, internal_server_error, method_not_allowed, not_found, not_implemented,
        unauthorized,
    },
};

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("query-server".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).unwrap();

    dotenv().ok();

    Env::validate(); // NOTE: Panic if the required env variables aren't set

    // NOTE: Create the config database
    create_config_db();

    let addr = SocketAddr::from(([0, 0, 0, 0], Env::port()));

    eprintln!("\nListening on {addr}\n");

    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async {
            let request_id = uuid::Uuid::new_v4().to_string();
            let span = tracing::info_span!("request", request_id = %request_id);
            let _enter = span.enter();

            Ok::<_, Infallible>(handle(req).instrument(span.clone()).await)
        }))
    });

    if let Err(err) = Server::bind(&addr).serve(make_service).await {
        tracing::error!("Server error: {err}");
    }
}

async fn handle(mut req: Request<Body>) -> Response<Body> {
    let path = req.uri().path().to_owned();
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    router(&mut req, &segments)
        .await
        .unwrap_or_else(|e| -> Response<Body> {
            tracing::error!("{}", e.to_string());

            match e.code {
                StatusCode::UNAUTHORIZED => unauthorized().unwrap(),
                StatusCode::BAD_REQUEST => bad_request(e.message.to_string()).unwrap(),
                StatusCode::METHOD_NOT_ALLOWED => method_not_allowed().unwrap(),
                StatusCode::NOT_IMPLEMENTED => not_implemented().unwrap(),
                StatusCode::NOT_FOUND => not_found().unwrap(),
                _ => internal_server_error(e.body).unwrap(),
            }
        })
}

async fn router(req: &mut Request<Body>, segments: &[&str]) -> Result<Response<Body>, HttpError> {
    if segments.is_empty() {
        return Err(HttpError {
            code: StatusCode::NOT_FOUND,
            message: StatusCode::NOT_FOUND.to_string(),
            body: None,
        });
    }

    match segments[0] {
        "migration" => migration::migration(req, segments).await,
        "query" => query::query(req, segments).await,
        "token" => token::token(req, segments).await,
        "user" => {
            if segments.len() > 1 && segments[1] == "token" {
                user_token::user_token(req, segments).await
            } else {
                user::user(req, segments).await
            }
        }
        _ => Err(HttpError {
            code: StatusCode::NOT_FOUND,
            message: StatusCode::NOT_FOUND.to_string(),
            body: None,
        }),
    }
}
