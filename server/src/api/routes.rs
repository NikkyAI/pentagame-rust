use super::errors::APIError;
use super::ws::GameServer;
use super::ws_session::WsGameSession;
use crate::auth::guard_api_with_user;
use crate::db::{actions::get_user_game, helper::acquire_connection_api, model::SlimUser};
use crate::frontend::routes::DbPool;
use actix::prelude::*;
use actix_web::HttpResponse;
use actix_web::{web::block, web::Data, web::Payload, HttpRequest};
use actix_web_actors::ws;
use std::time::Instant;

// General Response Type
pub type APIResponse = Result<HttpResponse, APIError>;

pub async fn game_route(
    id: Option<SlimUser>,
    pool: Data<DbPool>,
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<GameServer>>,
) -> APIResponse {
    let id = guard_api_with_user(id)?.id;
    let conn = acquire_connection_api(&pool)?;
    let result = block(move || get_user_game(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("The Database insertion failed unexpectedly");

    let gid = match result {
        Some(id) => id,
        None => {
            return Err(APIError::AuthorizationError {});
        }
    };

    APIError::wrap_error(
        ws::start(
            WsGameSession {
                id,
                game: gid,
                hb: Instant::now(),
                addr: srv.get_ref().clone(),
            },
            &req,
            stream,
        ),
        3,
    )
}
