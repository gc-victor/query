use http_body_util::BodyExt;
use hyper::{body::Incoming, Request, Response, Uri};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tracing::instrument;

use crate::env::Env;

use super::utils::{
    body::BoxBody,
    http_error::{bad_request, HttpError},
};

#[instrument(err(Debug), skip(req))]
pub async fn proxy(mut req: Request<Incoming>) -> Result<Response<BoxBody>, HttpError> {
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

    let client = Client::builder(TokioExecutor::new())
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let res = match client.request(req).await {
        Ok(res) => res.map(|body| body.boxed() as BoxBody),
        Err(e) => {
            return Err(HttpError {
                code: hyper::StatusCode::BAD_GATEWAY,
                message: e.to_string(),
                body: None,
            })
        }
    };

    Ok(res)
}
