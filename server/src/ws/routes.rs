use crate::api::errors::APIError;
use crate::auth::{guard_api_with_user, guard_with_user};
use crate::db::actions::{create_toast, get_user_game, leave_game};
use crate::db::model::SlimUser;
use crate::frontend::routes::{redirect, DbPool, UserResponse};
use crate::ws::{actor::GameServer, session::WsGameSession};
use actix::prelude::*;
use actix_web::{web::block, web::Data, web::HttpResponse, web::Payload, HttpRequest};
use actix_web_actors::ws;
use std::time::Instant;

pub async fn game_route(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<GameServer>>,
    pool: Data<DbPool>,
    id: Option<SlimUser>,
) -> Result<HttpResponse, APIError> {
    let user = guard_api_with_user(id)?;
    let conn = pool.get()?;

    /*
    this checks if the user already joined the game explicitly as the user might be only reconnecting
    */
    let sacrifice = user.id.clone();
    let result = block(move || get_user_game(&conn, sacrifice)).await?;

    let gid = match result {
        Some(id) => id,
        // check if game exists and if exists => join game
        None => {
            return Err(APIError::AuthorizationError(
                "You haven't joined this game. Consider visiting /game/view/{id} and checking out the game's data, if available.".to_owned()
            ));
        }
    };

    Ok(ws::start(
        WsGameSession {
            id: 0,
            uid: user,
            game: gid,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )?)
}

pub async fn get_game_leave_route(id: Option<SlimUser>, pool: Data<DbPool>) -> UserResponse {
    let user = guard_with_user(id)?;

    // leave game
    let conn = pool.get()?;
    let cloned_id = user.id.clone();

    block(move || leave_game(&conn, cloned_id)).await?;

    // alert user
    let conn = pool.get()?;
    block(move || create_toast(&conn, user.id, 1, "You left a game!".to_owned())).await?;

    Ok(redirect("/"))
}
