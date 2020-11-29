use crate::api::errors::APIError;
use crate::auth::guard_api_with_user;
use crate::db::model::SlimUser;
use crate::ws::{actor::GameServer, session::WsGameSession};
use actix::prelude::*;
use actix_web::{web::Data, web::HttpResponse, web::Payload, HttpRequest};
use actix_web_actors::ws;
use std::time::Instant;

pub async fn game_route(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<GameServer>>,
    id: Option<SlimUser>,
) -> Result<HttpResponse, APIError> {
    let user = guard_api_with_user(id)?;

    APIError::wrap_error(
        ws::start(
            WsGameSession {
                id: 0,
                cid: user,
                game: 0,
                hb: Instant::now(),
                addr: srv.get_ref().clone(),
            },
            &req,
            stream,
        ),
        3,
    )
}
