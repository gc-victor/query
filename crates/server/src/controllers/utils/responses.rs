use anyhow::Result;
use hyper::{Response, StatusCode};

use super::body::{Body, BoxBody};

pub fn ok<S: Into<String>>(body: S) -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body.into()))?)
}

pub fn created() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())?)
}

pub fn no_content() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())?)
}

pub fn unauthorized() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::empty())?)
}

pub fn not_found() -> Result<Response<BoxBody>> {
    let body = StatusCode::NOT_FOUND.to_string();

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(body))?)
}

pub fn method_not_allowed() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())?)
}

pub fn not_implemented() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(Body::empty())?)
}

pub fn bad_request(text: String) -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(text))?)
}

pub fn internal_server_error(body: Option<String>) -> Result<Response<BoxBody>> {
    let body = body.unwrap_or_else(|| "Internal Server Error".to_string());

    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(body))?)
}

#[cfg(test)]
mod tests {
    use hyper::StatusCode;

    use super::*;

    #[test]
    fn test_response_with_ok() {
        let resp = ok("test").unwrap();

        assert_eq!(
            format!("{:?}", Body::from("test")),
            format!("{:?}", resp.body())
        );

        assert_eq!(
            format!("{:?}", StatusCode::OK),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_created() {
        let resp = created().unwrap();

        assert_eq!(format!("{:?}", Body::empty()), format!("{:?}", resp.body()));

        assert_eq!(
            format!("{:?}", StatusCode::CREATED),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_no_content() {
        let resp = no_content().unwrap();

        assert_eq!(format!("{:?}", Body::empty()), format!("{:?}", resp.body()));

        assert_eq!(
            format!("{:?}", StatusCode::NO_CONTENT),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_unauthorized() {
        let resp = unauthorized().unwrap();

        assert_eq!(format!("{:?}", Body::empty()), format!("{:?}", resp.body()));

        assert_eq!(
            format!("{:?}", StatusCode::UNAUTHORIZED),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_not_found() {
        let resp = not_found().unwrap();

        assert_eq!(
            format!("{:?}", Body::from(StatusCode::NOT_FOUND.to_string())),
            format!("{:?}", resp.body())
        );

        assert_eq!(
            format!("{:?}", StatusCode::NOT_FOUND),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_method_not_allowed() {
        let resp = method_not_allowed().unwrap();

        assert_eq!(format!("{:?}", Body::empty()), format!("{:?}", resp.body()));

        assert_eq!(
            format!("{:?}", StatusCode::METHOD_NOT_ALLOWED),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_not_implemented() {
        let resp = not_implemented().unwrap();

        assert_eq!(format!("{:?}", Body::empty()), format!("{:?}", resp.body()));
        assert_eq!(
            format!("{:?}", StatusCode::NOT_IMPLEMENTED),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_bad_request() {
        let resp = bad_request("test".to_string()).unwrap();

        assert_eq!(
            format!("{:?}", Body::from("test")),
            format!("{:?}", resp.body())
        );

        assert_eq!(
            format!("{:?}", StatusCode::BAD_REQUEST),
            format!("{:?}", resp.status())
        );
    }

    #[test]
    fn test_response_with_internal_server_error() {
        let resp = internal_server_error(None).unwrap();

        assert_eq!(
            format!("{:?}", Body::from("Internal Server Error")),
            format!("{:?}", resp.body())
        );

        assert_eq!(
            format!("{:?}", StatusCode::INTERNAL_SERVER_ERROR),
            format!("{:?}", resp.status())
        );
    }
}
