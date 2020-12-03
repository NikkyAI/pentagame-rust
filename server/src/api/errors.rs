// imports
use actix::dev::SendError;
use actix_web::{
    dev::HttpResponseBuilder, error::BlockingError, error::Error as WebError, error::ResponseError,
    http::header, http::StatusCode, HttpResponse,
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use serde_json::Error as SerializeError;
use uuid::ParseError;

/*
APIError:
    displays user when something is not e.g. available as html for GET Requests
    returns template error.html

    Errors:
    ValidationError: For login/ authenticated queries and requests. Acts as fallback for malformed requests. The UI should tank most validation
    InternalError: Something went really, really wrong
    DataBasePoolError: the pool is exhausted (hopefully) only for the moment
*/
#[derive(Debug, Display)]
pub enum APIError {
    ValidationError(String),
    InternalError(String),
    PoolError(String),
    AuthorizationError(String),
    BlockingError(String),
    IPCError(String),
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            APIError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::PoolError { .. } => StatusCode::SERVICE_UNAVAILABLE,
            APIError::AuthorizationError { .. } => StatusCode::UNAUTHORIZED,
            APIError::BlockingError { .. } => StatusCode::TOO_MANY_REQUESTS,
            /*
             When the threads can't communicate one of your actors most likely died on you.
             At this point I probably fcked up the error handling and should asap upload a fix
            */
            APIError::IPCError { .. } => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl From<PoolError> for APIError {
    fn from(error: PoolError) -> APIError {
        APIError::PoolError(error.to_string())
    }
}

// Convert ParseErrors/ SerializeError to APIErrors
impl From<ParseError> for APIError {
    fn from(error: ParseError) -> APIError {
        APIError::ValidationError(error.to_string())
    }
}

impl From<SerializeError> for APIError {
    fn from(error: SerializeError) -> APIError {
        APIError::InternalError(error.to_string())
    }
}

// convert WebErrors to APIErrors
impl From<WebError> for APIError {
    fn from(error: WebError) -> APIError {
        APIError::InternalError(error.to_string())
    }
}

// Convert SendError to APIError
impl<T> From<SendError<T>> for APIError {
    fn from(error: SendError<T>) -> APIError {
        APIError::IPCError(format!(
            "Couldn't deliver actor message: {}",
            error.to_string()
        ))
    }
}

// Convert Thread BlockingErrors to APIErrors
impl From<BlockingError<WebError>> for APIError {
    fn from(error: BlockingError<WebError>) -> APIError {
        match error {
            BlockingError::Error(web_error) => APIError::InternalError(web_error.to_string()),
            BlockingError::Canceled => APIError::BlockingError("Thread blocking error".into()),
        }
    }
}

impl From<BlockingError<APIError>> for APIError {
    fn from(error: BlockingError<APIError>) -> APIError {
        match error {
            BlockingError::Error(user_error) => user_error,
            BlockingError::Canceled => APIError::BlockingError("Thread blocking error".into()),
        }
    }
}

impl From<BlockingError<DBError>> for APIError {
    fn from(error: BlockingError<DBError>) -> APIError {
        match error {
            BlockingError::Error(db_error) => APIError::InternalError(db_error.to_string()),
            BlockingError::Canceled => APIError::BlockingError("Thread blocking error".into()),
        }
    }
}

// Convert DBErrors to APIErrors
impl From<DBError> for APIError {
    fn from(error: DBError) -> APIError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return APIError::ValidationError(message);
                }
                APIError::InternalError("Unknown database error".to_owned())
            }
            DBError::NotFound { .. } => APIError::ValidationError(
                "Requested Item or resource was not found/ is not available".to_owned(),
            ),
            _ => APIError::InternalError("Unknown database error".to_owned()),
        }
    }
}
