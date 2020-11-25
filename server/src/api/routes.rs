use super::errors::APIError;
use super::ws::GameServer;
use super::ws_session::WsGameSession;
use actix::prelude::*;
use actix_web::{HttpResponse, error::Error as WebError, web::Data, web::Payload, HttpRequest};
use actix_web_actors::ws;
use std::time::Instant;

// General Response Type
// pub type APIResponse = Result<HttpResponse, APIError>;

pub async fn game_route(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<GameServer>>,
) -> Result<HttpResponse, WebError> {
    println!("Recieved something");
    ws::start(
        WsGameSession {
            id: 0,
            game: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
