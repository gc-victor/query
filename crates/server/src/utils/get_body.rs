use hyper::{body::to_bytes, Body, Request};
use regex::Regex;
use tracing::instrument;

use crate::{
    utils::http_error::{bad_request, internal_server_error},
    HttpError,
};

#[instrument(err(Debug), skip(req))]
pub async fn get_body(req: &mut Request<Body>) -> Result<String, HttpError> {
    let body = match stringify_body(req).await {
        Ok(value) => value,
        Err(e) => return Err(internal_server_error(e)),
    };

    if body.is_empty() {
        return Err(bad_request("The body is empty".to_string()));
    }

    let re = Regex::new(r"[\x00-\x1F]").unwrap();
    let body = re.replace_all(&body, "").to_string();

    Ok(body)
}

async fn stringify_body(req: &mut Request<Body>) -> Result<String, String> {
    let bytes = match to_bytes(req.body_mut()).await {
        Ok(bytes) => bytes,
        Err(e) => return Err(e.to_string()),
    };

    let body = match String::from_utf8(bytes.to_vec()) {
        Ok(body) => body,
        Err(e) => return Err(e.to_string()),
    };

    Ok(body)
}

#[cfg(test)]
mod tests {
    use hyper::{Body, Request, StatusCode};

    use crate::utils::get_body::get_body;
    use crate::HttpError;

    #[tokio::test]
    async fn test_get_the_body_from_a_request() {
        let body_str = "SELECT * FROM table;";
        let mut req: Request<Body> = Request::new(Body::from(body_str));

        assert_eq!(body_str, get_body(&mut req).await.unwrap());
    }

    #[tokio::test]
    async fn test_get_a_bad_request_error_if_the_body_is_empty() {
        let body_str = "";
        let mut req: Request<Body> = Request::new(Body::from(body_str));

        let err = match get_body(&mut req).await {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
        .unwrap_err();

        assert_eq!(
            HttpError {
                code: StatusCode::BAD_REQUEST,
                message: "The body is empty".to_string(),
                body: None,
            },
            err
        );
    }

    #[tokio::test]
    async fn test_get_a_internal_server_error_with_invalid_utf_8_sequence() {
        let body_str = vec![0, 159, 146, 150];
        let mut req: Request<Body> = Request::new(Body::from(body_str));

        let err = match get_body(&mut req).await {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
        .unwrap_err();

        assert_eq!(
            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "invalid utf-8 sequence of 1 bytes from index 1".to_string(),
                body: None,
            },
            err
        )
    }
}
