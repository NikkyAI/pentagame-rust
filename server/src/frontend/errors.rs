// imports
use super::routes::{redirect, UserResponse};
use super::templates;
use actix_web::{
    dev::HttpResponseBuilder, error::BlockingError, error::ResponseError, http::header,
    http::StatusCode, Error as WebError, HttpResponse,
};
use askama_actix::TemplateIntoResponse;
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use uuid::ParseError;

/*
UserError:
    displays user when something is not e.g. available as html for GET Requests
    returns template error.html

    Errors:
    ValidationError: Only returned for non-auth queries as everything else is API (POST) based
    InternalError: Something went really, really wrong

    codes:
        1: not found
        2: LOL
*/
#[derive(Debug, Display)]
pub enum UserError {
    PoolError(String),
    InternalError(String),
    AuthorizationError(String),
    ValidationError(String),
    BlockingError(String),
    NotFoundError(),
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        eprintln!("UserError: {:?}", self.to_string());

        let response = match self.status_code() {
            StatusCode::UNAUTHORIZED => Ok(redirect("/users/login")),
            StatusCode::INTERNAL_SERVER_ERROR => templates::ErrorTemplate {
                message: self.into(),
                code: 500,
                id: None
            }
            .into_response(),
            StatusCode::BAD_REQUEST => templates::ErrorTemplate {
                message: self.into(),
                code: 400,
                id: None
            }.into_response(),
            StatusCode::NOT_FOUND => templates::ErrorTemplate {
                message: self.into(),
                code: 404,
                id: None
            }
            .into_response(),
            _ => templates::ErrorTemplate {
                message: format!("Something went terribly wrong on our side. We are sorry for any caused inconvenience."),
                code: 500,
                id: None
            }
            .into_response(),
        };

        match response {
            Ok(resp) => resp,
            Err(why) => {
                eprintln!("UserError: {:?}", why);
                HttpResponseBuilder::new(self.status_code())
                    .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body(self.to_string())
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::AuthorizationError { .. } => StatusCode::UNAUTHORIZED,
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::NotFoundError { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl UserError {
    pub fn wrap_template(res: Result<HttpResponse, WebError>) -> UserResponse {
        match res {
            Ok(response) => Ok(response),
            Err(why) => {
                eprintln!("InternalError: {:?}", why);
                Err(UserError::InternalError(
                    "Failed to render requested template".to_owned(),
                ))
            }
        }
    }
}

// Convert PoolErrors to UserErrors
impl From<PoolError> for UserError {
    fn from(error: PoolError) -> UserError {
        UserError::PoolError(error.to_string())
    }
}

// Convert ParseErrors to UserErrors
impl From<ParseError> for UserError {
    fn from(error: ParseError) -> UserError {
        UserError::ValidationError(error.to_string())
    }
}

// convert WebErrors to UserErrors
impl From<WebError> for UserError {
    fn from(error: WebError) -> UserError {
        UserError::InternalError(error.to_string())
    }
}

// Convert Thread BlockingErrors to UserErrors
impl From<BlockingError<WebError>> for UserError {
    fn from(error: BlockingError<WebError>) -> UserError {
        match error {
            BlockingError::Error(web_error) => UserError::InternalError(web_error.to_string()),
            BlockingError::Canceled => UserError::BlockingError("Thread blocking error".into()),
        }
    }
}

impl From<BlockingError<UserError>> for UserError {
    fn from(error: BlockingError<UserError>) -> UserError {
        match error {
            BlockingError::Error(user_error) => user_error,
            BlockingError::Canceled => UserError::BlockingError("Thread blocking error".into()),
        }
    }
}

impl From<BlockingError<DBError>> for UserError {
    fn from(error: BlockingError<DBError>) -> UserError {
        match error {
            BlockingError::Error(db_error) => UserError::from(db_error),
            BlockingError::Canceled => UserError::BlockingError("Thread blocking error".into()),
        }
    }
}

// Convert DBErrors to UserErrors
impl From<DBError> for UserError {
    fn from(error: DBError) -> UserError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return UserError::ValidationError(message);
                }
                UserError::InternalError("Unknown database error".to_owned())
            }
            DBError::NotFound { .. } => UserError::NotFoundError(),
            _ => UserError::InternalError("Unknown database error".to_owned()),
        }
    }
}

// String casting for UserErrors
impl From<UserError> for String {
    fn from(error: UserError) -> String {
        match error {
            UserError::NotFoundError { .. } => "Didn't found the requested resource".to_owned(),
            UserError::PoolError(message) => message,
            UserError::InternalError(message) => message,
            UserError::AuthorizationError(message) => message,
            UserError::ValidationError(message) => message,
            UserError::BlockingError(message) => message,
        }
    }
}

impl<'a> From<&'a UserError> for String {
    fn from(error: &'a UserError) -> String {
        match error {
            UserError::NotFoundError { .. } => "Didn't found the requested resource".to_owned(),
            UserError::PoolError(message) => message.to_owned(),
            UserError::InternalError(message) => message.to_owned(),
            UserError::AuthorizationError(message) => message.to_owned(),
            UserError::ValidationError(message) => message.to_owned(),
            UserError::BlockingError(message) => message.to_owned(),
        }
    }
}
