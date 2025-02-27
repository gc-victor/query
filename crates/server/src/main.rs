pub mod constants;

pub mod controllers;
pub mod env;
pub mod sqlite;

use std::convert::Infallible;
use std::net::SocketAddr;

use controllers::cache_manager::start_invalidation_task;
use dotenv::dotenv;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use sqlite::create_cache_invalidation_db::create_cache_invalidation_db;
use tokio::net::TcpListener;
use tracing::{subscriber::set_global_default, Instrument};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, Registry};

use crate::{
    controllers::{
        asset::asset,
        asset_builder::asset_builder,
        branch::branch,
        function::function,
        function_builder::function_builder,
        migration::migration,
        plugin_builder::plugin_builder,
        proxy::proxy,
        query::query,
        token::token,
        user::user,
        user_token::user_token,
        utils::{
            body::{Body, BoxBody},
            http_error::HttpError,
            responses::{
                bad_request, internal_server_error, method_not_allowed, not_found, not_implemented,
                unauthorized,
            },
        },
    },
    env::Env,
    sqlite::{
        create_asset_db::create_asset_db, create_config_db::create_config_db,
        create_function_db::create_function_db, create_plugin_db::create_plugin_db,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,query-runtime=info"));
    let formatting_layer = BunyanFormattingLayer::new("query-server".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).unwrap();

    dotenv().ok();

    Env::validate(); // NOTE: Panic if the required env variables aren't set

    // NOTE: Create the asset database
    create_asset_db();
    // NOTE: Create the create_cache_invaildation_db database
    create_cache_invalidation_db();
    // NOTE: Create the config database
    create_config_db();
    // NOTE: Create the function database
    create_function_db();
    // NOTE: Create the plugin database
    create_plugin_db();
    // NOTE: Start the cache invalidation task
    start_invalidation_task();

    let addr = SocketAddr::from(([0, 0, 0, 0], Env::port()));
    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(
        init = true,
        message = format!("\nListening on http://{addr} - v{VERSION}\n")
    );

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| async {
                let request_id = uuid::Uuid::new_v4().to_string();
                let span = tracing::info_span!("request", request_id = %request_id);
                let _enter = span.enter();

                Ok::<_, Infallible>(handler(req).instrument(span.clone()).await)
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                tracing::error!("Server error: {err}");
            }
        });
    }
}

async fn handler(req: Request<IncomingBody>) -> Response<BoxBody> {
    let path = req.uri().path().to_owned();
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    router(req, &segments)
        .await
        .unwrap_or_else(|e: HttpError| -> Response<BoxBody> {
            let code = e.code.as_u16();
            let error = e.to_string();
            tracing::error!(code, path, "{error}");

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

async fn router(
    req: Request<IncomingBody>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    if segments.is_empty() {
        if Env::proxy() == "true" {
            return proxy(req).await;
        }

        if Env::app() == "true" {
            let mut req = req;

            return function(&mut req).await;
        }

        return Err(HttpError {
            code: StatusCode::NOT_FOUND,
            message: StatusCode::NOT_FOUND.to_string(),
            body: None,
        });
    }

    let init_segment = segments[0];
    let segments = &segments[1..];

    let mut req = req;

    match init_segment {
        "_" => match segments[0] {
            "asset" => asset(&mut req, segments).await,
            "asset-builder" => asset_builder(&mut req, segments).await,
            "branch" => branch(&mut req, segments).await,
            "function" => function(&mut req).await,
            "function-builder" => function_builder(&mut req, segments).await,
            "healthcheck" => Ok(Response::new(Body::from("OK"))),
            "migration" => migration(&mut req, segments).await,
            "plugin-builder" => plugin_builder(&mut req, segments).await,
            "query" => query(&mut req, segments).await,
            "token" => token(&mut req, segments).await,
            "user" => {
                if segments.len() > 1 && segments[1] == "token" {
                    user_token(&mut req, segments).await
                } else {
                    user(&mut req, segments).await
                }
            }
            _ => Err(HttpError {
                code: StatusCode::NOT_FOUND,
                message: StatusCode::NOT_FOUND.to_string(),
                body: None,
            }),
        },
        _ => {
            if Env::proxy() == "true" {
                return proxy(req).await;
            }

            if Env::app() == "true" {
                return function(&mut req).await;
            }

            Err(HttpError {
                code: StatusCode::NOT_FOUND,
                message: StatusCode::NOT_FOUND.to_string(),
                body: None,
            })
        }
    }
}
