use hyper::{Body, Client, Request, Response};
use tracing::instrument;

use crate::env::Env;

use super::utils::http_error::HttpError;

#[instrument(err(Debug), skip(req))]
pub async fn proxy(req: &mut Request<Body>) -> Result<Response<Body>, HttpError> {
    let path = req.uri().path().to_string();
    let target_url = format!("http://localhost:{}{}", Env::proxy_port(), path);
    let headers = req.headers().clone();
    let body_bytes = match hyper::body::to_bytes(req.body_mut()).await {
        Ok(body_bytes) => body_bytes,
        Err(e) => {
            return Err(HttpError {
                code: hyper::StatusCode::BAD_GATEWAY,
                message: e.to_string(),
                body: None,
            })
        }
    };
    let mut request_builder = Request::builder()
        .method(req.method())
        .uri(target_url)
        .body(Body::from(body_bytes))
        .unwrap();

    *request_builder.headers_mut() = headers;

    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let response = match client.request(request_builder).await {
        Ok(response) => response,
        Err(e) => {
            return Err(HttpError {
                code: hyper::StatusCode::BAD_GATEWAY,
                message: e.to_string(),
                body: None,
            })
        }
    };

    Ok(response)
}
