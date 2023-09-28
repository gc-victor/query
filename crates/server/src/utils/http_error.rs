use std::{
    error::{self, Error},
    fmt,
};

use hyper::StatusCode;

#[derive(PartialEq, Eq)]
pub struct HttpError {
    pub code: StatusCode,
    pub message: String,
    pub body: Option<String>,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let unauthorized = unauthorized();
        let unauthorized_message = unauthorized.message;

        let bad_request = bad_request("Bad Request".to_string());
        let bad_request_message = bad_request.message;

        let internal_server = internal_server_error("Internal Server Error".to_string());
        let internal_server_message = internal_server.message;

        let not_implemented = not_implemented();
        let not_implemented_message = not_implemented.message;

        let not_found = not_found();
        let not_found_message = not_found.message;

        let err_msg = match self.code {
            StatusCode::UNAUTHORIZED => unauthorized_message,
            StatusCode::BAD_REQUEST => bad_request_message,
            StatusCode::INTERNAL_SERVER_ERROR => internal_server_message,
            StatusCode::NOT_IMPLEMENTED => not_implemented_message,
            StatusCode::NOT_FOUND => not_found_message,
            _ => "Sorry, something is wrong! Please Try Again!".to_string(),
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HttpError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

impl Error for HttpError {}

impl From<Box<dyn error::Error>> for HttpError {
    fn from(e: Box<dyn error::Error>) -> Self {
        internal_server_error(e.to_string())
    }
}

impl From<rusqlite::Error> for HttpError {
    fn from(e: rusqlite::Error) -> Self {
        bad_request(e.to_string())
    }
}

impl From<anyhow::Error> for HttpError {
    fn from(e: anyhow::Error) -> Self {
        internal_server_error(e.to_string())
    }
}

pub fn unauthorized() -> HttpError {
    HttpError {
        code: StatusCode::UNAUTHORIZED,
        message: "Unauthorized".to_string(),
        body: None,
    }
}

pub fn bad_request(e: String) -> HttpError {
    HttpError {
        code: StatusCode::BAD_REQUEST,
        message: e,
        body: None,
    }
}

pub fn internal_server_error(e: String) -> HttpError {
    HttpError {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        message: e.to_string(),
        body: None,
    }
}

pub fn not_implemented() -> HttpError {
    HttpError {
        code: StatusCode::NOT_IMPLEMENTED,
        message: "Not Implemented".to_string(),
        body: None,
    }
}

pub fn not_found() -> HttpError {
    HttpError {
        code: StatusCode::NOT_FOUND,
        message: "Not Found".to_string(),
        body: None,
    }
}

#[cfg(test)]
mod tests {
    use hyper::StatusCode;

    use super::*;

    #[test]
    fn test_return_display_formatted() {
        let unauthorized = unauthorized();
        let bad_request = bad_request("Bad Request".to_string());
        let not_implemented = not_implemented();
        let not_found = not_found();
        let internal_server_error = internal_server_error("".to_string());
        let unknown = HttpError {
            code: StatusCode::IM_A_TEAPOT,
            message: "Something".to_string(),
            body: None,
        };

        assert_eq!("Not Implemented", format!("{not_implemented}"));
        assert_eq!("Internal Server Error", format!("{internal_server_error}"));
        assert_eq!("Not Found", format!("{not_found}"));
        assert_eq!("Bad Request", format!("{bad_request}"));
        assert_eq!("Unauthorized", format!("{unauthorized}"));
        assert_eq!(
            "Sorry, something is wrong! Please Try Again!",
            format!("{unknown}")
        );
    }

    #[test]
    fn test_return_debug_formatted() {
        let error = unauthorized();

        assert_eq!(
            "HttpError { code: 401 Unauthorized, message: Unauthorized }".to_string(),
            format!("{:?}", error)
        );
    }

    #[test]
    fn test_from_rusqlite_error() {
        let e = rusqlite::Error::QueryReturnedNoRows;
        let http_error = HttpError::from(e);

        assert_eq!(http_error.code, StatusCode::BAD_REQUEST);
        assert_eq!(http_error.message, "Query returned no rows".to_string());
    }

    #[test]
    fn test_return_not_found_error() {
        let error = not_found();

        assert_eq!(404, error.code);
        assert_eq!("Not Found".to_string(), error.message);
    }

    #[test]
    fn test_return_unauthorized_error() {
        let error = unauthorized();

        assert_eq!(401, error.code);
        assert_eq!("Unauthorized".to_string(), error.message);
    }

    #[test]
    fn test_return_not_implemented_error() {
        let error = not_implemented();

        assert_eq!(501, error.code);
        assert_eq!("Not Implemented".to_string(), error.message);
    }

    #[test]
    fn test_return_bad_request_error() {
        let error = bad_request("Bad Request Test".to_string());

        assert_eq!(StatusCode::BAD_REQUEST, error.code);
        assert_eq!("Bad Request Test", error.message);
    }

    #[test]
    fn test_return_internal_server_error_error() {
        let error = internal_server_error("Internal Server Error Test".to_string());

        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, error.code);
        assert_eq!("Internal Server Error Test", error.message);
    }
}
