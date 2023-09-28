use hyper::{header::AUTHORIZATION, Body, Request};
use tracing::instrument;

use crate::{utils::http_error::unauthorized, HttpError};

#[instrument(err(Debug), skip(req))]
pub fn get_token(req: &Request<Body>) -> Result<String, HttpError> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(unauthorized)?
        .to_str()
        .map_err(|_| unauthorized())?
        .split(' ')
        .nth(1)
        .ok_or_else(unauthorized)?;

    if token.is_empty() {
        tracing::error!("Empty token");
        return Err(unauthorized());
    }

    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use hyper::header;

    use super::*;

    #[test]
    fn test_get_token() {
        let req = Request::builder()
            .header(header::AUTHORIZATION, "Bearer token")
            .body(Body::empty())
            .unwrap();
        let result = get_token(&req);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "token");
    }

    #[test]
    fn test_invalid_authorization_header() {
        let req = Request::builder()
            .header(header::AUTHORIZATION, "invalid_header")
            .body(Body::empty())
            .unwrap();
        let result = get_token(&req);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }

    #[test]
    fn test_missing_authorization_header() {
        let req = Request::builder().body(Body::empty()).unwrap();
        let result = get_token(&req);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }

    #[test]
    fn test_missing_token() {
        let req = Request::builder()
            .header(header::AUTHORIZATION, "Bearer ")
            .body(Body::empty())
            .unwrap();
        let result = get_token(&req);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }
}
