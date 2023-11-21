use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use regex::Regex;

use super::http_error::{bad_request, internal_server_error, HttpError};

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub struct Body;

impl Body {
    pub fn empty() -> BoxBody {
        Body::from("")
    }

    pub fn from<T: Into<Bytes>>(chunk: T) -> BoxBody {
        Full::new(chunk.into())
            .map_err(|never| match never {})
            .boxed()
    }

    pub async fn to_bytes(incoming_body: &mut Incoming) -> Result<Bytes, HttpError> {
        match BodyExt::collect(incoming_body).await {
            Ok(c) => Ok(c.to_bytes()),
            Err(e) => return Err(internal_server_error(e.to_string())),
        }
    }

    pub async fn to_string(incoming_body: &mut Incoming) -> Result<String, HttpError> {
        let bytes = Body::to_bytes(incoming_body).await?;
        let body = match String::from_utf8(bytes.to_vec()) {
            Ok(body) => body,
            Err(e) => return Err(internal_server_error(e.to_string())),
        };

        if body.is_empty() {
            return Err(bad_request("The body is empty".to_string()));
        }

        let re = Regex::new(r"[\x00-\x1F]").unwrap();
        let body = re.replace_all(&body, "").to_string();

        Ok(body)
    }
}
