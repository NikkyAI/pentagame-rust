// imports
use actix_web::{
    dev::HttpResponseBuilder, error::ResponseError, http::header, http::StatusCode,
    Error as WebError, HttpResponse,
};
use derive_more::{Display, Error};

/*
APIError:
    displays user when something is not e.g. available as html for GET Requests
    returns template error.html

    Errors:
    ValidationError: For login/ authenticated queries and requests. Acts as fallback for malformed requests. The UI should tank most validation
    InternalError: Something went really, really wrong
    DataBasePoolError: the pool is exhausted (hopefully) only for the moment
*/
#[derive(Debug, Display, Error)]
pub enum APIError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "Internal Error {}: {}", code, message)]
    InternalError { code: u16, message: String },
    #[display(fmt = "Database exhausted: {}", message)]
    DataBasePoolError { message: String },
    #[display(fmt = "Unauthorized Access")]
    AuthorizationError {},
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        eprintln!("{}", self.to_string());
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            APIError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::DataBasePoolError { .. } => StatusCode::SERVICE_UNAVAILABLE,
            APIError::AuthorizationError { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}

impl APIError {
    // this is meant for functions that may return HTTPResponse
    pub fn wrap_response<T>(res: Result<T, HttpResponse>, code: u16) -> Result<T, APIError> {
        match res {
            Ok(response) => Ok(response),
            Err(_) => Err(APIError::InternalError {
                code: code,
                message: "Internal Error".to_string(),
            }),
        }
    }
}
