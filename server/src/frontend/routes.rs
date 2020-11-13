// imports
use super::errors::UserError;
use super::templates;
use crate::db::helper::acquire_connection_user;
use actix_web::{dev::HttpResponseBuilder, http::header, http::StatusCode, HttpResponse};
#[allow(unused_imports)] // Required as trait in scope for template.into_response()
use askama_actix::TemplateIntoResponse;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use serde::Serialize;

// types
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type APIResponse = Result<HttpResponse, APIError>;
pub type UserResponse = Result<HttpResponse, UserError>;

// Simple Redirect
pub fn redirect<'a>(route: &'a str) -> HttpResponse {
    HttpResponseBuilder::new(StatusCode::SEE_OTHER)
        .header(header::LOCATION, route)
        .finish()
}

// empty string constant
const EMPTY: &'static str = "";

/*
General API Responses
    ActionStatus:
        code:
            0: Success
            1: Failure
            2: invalid
            3: unauthorized
        description: Description of err/ success

    QueryResult:
        code:
            Same as ActionStatus
        data:
            Post (id, title, body, published)
*/

#[derive(Serialize)]
pub struct ActionStatus {
    code: i8,
    description: String,
}

#[derive(Serialize)]
pub struct QueryResult {
    code: i8,
    data: (u32, String, String, bool),
}

/*
General:
/ -> get_index
/cookies -> get_cookie_information

Not registered -> get_error_404
*/

pub async fn get_index() -> UserResponse {
    UserError::wrap_template(templates::IndexTemplate {}.into_response())
}

pub async fn get_rules() -> UserResponse {
    UserError::wrap_template(templates::RulesTemplate {}.into_response())
}

pub async fn get_error_404() -> UserResponse {
    UserError::wrap_template(
        templates::ErrorTemplate {
            code: 404,
            message: "The requested page is not available".to_owned(),
        }
        .into_response(),
    )
}
