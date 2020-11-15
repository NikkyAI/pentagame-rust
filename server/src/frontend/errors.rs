// imports
use super::routes::{redirect, UserResponse};
use super::templates;
use actix_web::{
    dev::HttpResponseBuilder, error::ResponseError, http::header, http::StatusCode,
    Error as WebError, HttpResponse,
};
use askama_actix::TemplateIntoResponse;
use derive_more::{Display, Error};

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
#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "Internal Error {}: {}", code, message)]
    InternalError { code: u16, message: String },
    #[display(fmt = "Query Error {}: {}", code, message)]
    QueryError { code: u16, message: String },
    #[display(fmt = "Authorization is required")]
    AuthorizationError {},
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let response = match self.status_code() {
            StatusCode::UNAUTHORIZED => Ok(redirect("/users/login")),
            StatusCode::INTERNAL_SERVER_ERROR => templates::ErrorTemplate {
                message: format!("Something went terribly wrong on our side. We are sorry for any caused inconvenience."),
                code: 500,
                id: None
            }
            .into_response(),
            StatusCode::BAD_REQUEST => templates::ErrorTemplate {
                message: "Seems like you submitted a corrupted/ invalid form".to_owned(),
                code: 400,
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
            Err(_) => HttpResponseBuilder::new(self.status_code())
                .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(self.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::QueryError { .. } => StatusCode::NOT_FOUND,
            UserError::AuthorizationError { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}

impl UserError {
    pub fn wrap_template(res: Result<HttpResponse, WebError>) -> UserResponse {
        match res {
            Ok(response) => Ok(response),
            Err(_) => Err(UserError::InternalError {
                code: 1,
                message: "Failed to render requested template".to_owned(),
            }),
        }
    }
}
