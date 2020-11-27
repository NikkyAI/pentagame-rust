use crate::auth::guard_api_with_user;
use crate::db::model::SlimUser;
use crate::ws::{actor::GameServer, session::WsGameSession};
use actix::prelude::*;
use actix_web::{
    error::Error as WebError, web::Data, web::HttpResponse, web::Payload, HttpRequest,
};
use actix_web_actors::ws;
use std::time::Instant;

pub async fn game_route(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<GameServer>>,
    id: Option<SlimUser>,
) -> Result<HttpResponse, WebError> {
    let check = guard_api_with_user(id);

    let user = match check {
        Ok(user) => user,
        Err(e) => {
            return Err(actix_web::error::ErrorForbidden(e));
        }
    };

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
    )
}
