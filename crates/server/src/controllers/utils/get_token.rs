use hyper::{header::AUTHORIZATION, HeaderMap};
use tracing::instrument;

use super::http_error::{unauthorized, HttpError};

#[instrument(err(Debug), skip(headers))]
pub fn get_token(headers: HeaderMap) -> Result<String, HttpError> {
    let token = headers
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
    use hyper::header::{self, HeaderValue};

    use super::*;

    #[test]
    fn test_get_token() {
        let mut headers = HeaderMap::new();

        headers.append(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer token"),
        );

        let result = get_token(headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "token");
    }

    #[test]
    fn test_invalid_authorization_header() {
        let mut headers = HeaderMap::new();

        headers.append(
            header::AUTHORIZATION,
            HeaderValue::from_static("invalid_value"),
        );
        let result = get_token(headers);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }

    #[test]
    fn test_missing_authorization_header() {
        let headers = HeaderMap::new();
        let result = get_token(headers);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }

    #[test]
    fn test_missing_token() {
        let mut headers = HeaderMap::new();

        headers.append(header::AUTHORIZATION, HeaderValue::from_static("Bearer "));
        let result = get_token(headers);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }
}
