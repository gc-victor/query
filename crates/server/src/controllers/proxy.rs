use hyper::{Body, Client, Request, Response, Uri};
use tracing::instrument;

use crate::env::Env;

use super::utils::http_error::{bad_request, HttpError};

#[instrument(err(Debug), skip(req))]
pub async fn proxy(mut req: Request<Body>) -> Result<Response<Body>, HttpError> {
    let path = req.uri().path().to_string();
    let query = match req.uri().query() {
        Some(query) => format!("?{}", query),
        None => "".to_string(),
    };

    *req.uri_mut() = match format!("http://localhost:{}{}{}", Env::proxy_port(), path, query)
        .as_str()
        .parse::<Uri>()
    {
        Ok(u) => Ok(u),
        Err(e) => Err(bad_request(e.to_string())),
    }?;

    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let response = match client.request(req).await {
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
