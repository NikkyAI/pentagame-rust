use super::errors::APIError;
use super::requests::{GetGameRequest, PostLoginRequest};
use super::responses::{GetGameResponse, PostLoginResponse};
use super::ws::GameServer;
use super::ws_session::WsGameSession;
use crate::auth::verify_hash;
use crate::db::actions::{get_game, get_user_by_username};
use crate::db::{helper::acquire_connection_api, model::SlimUser};
use crate::frontend::routes::DbPool;
use actix::prelude::*;
use actix_identity::Identity;
use actix_web::{
    error::Error as WebError, web::block, web::Data, web::Json, web::Payload, HttpRequest,
    HttpResponse,
};
use actix_web_actors::ws;
use std::time::Instant;

// General Response Type
pub type APIResponse = Result<HttpResponse, APIError>;
// pub type APIResult<T> = Result<T, APIError>;

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

pub async fn get_game_meta(
    id: Option<SlimUser>,
    pool: Data<DbPool>,
    data: Json<GetGameRequest>,
) -> APIResponse {
    let conn = acquire_connection_api(&pool)?;

    let result = block(move || get_game(&conn, data.id)).await.map_err(|e| {
        eprintln!("{}", e);
        return APIError::InternalError {
            code: 1,
            message: "Something unexpected happened".to_owned(),
        };
    })?;

    match result {
        Some(game) => {
            if game.0.public {
                Ok(HttpResponse::Ok().json(GetGameResponse {
                    game: game.0,
                    users: game.1,
                }))
            } else {
                Err(APIError::ValidationError {
                    field: "id".to_owned(),
                })
            }
        }
        None => Err(APIError::ValidationError {
            field: "id".to_owned(),
        }),
    }
}

pub async fn post_login(
    id: Identity,
    pool: Data<DbPool>,
    data: Json<PostLoginRequest>,
) -> APIResponse {
    // validates cookie checkbox
    if !data.0.cookie {
        return Err(APIError::ValidationError {
            field: "cookie".to_owned(),
        });
    }
    // acquiring connection from db pool
    let conn = acquire_connection_api(&pool)?;

    let sacrifice = data.username.clone();
    let result = block(move || get_user_by_username(&conn, sacrifice))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            APIError::InternalError {
                code: 1,
                message: "Something unexpected happened".to_owned(),
            }
        })?;

    let user = match result {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Ok().json(PostLoginResponse {
                authenticated: false,
            }));
        }
    };

    // verify hash and remember serialized Slimuser
    if verify_hash(&user.password, &data.password) {
        let user_string = serde_json::to_string(&SlimUser::from(user)).unwrap();
        id.remember(user_string);

        Ok(HttpResponse::Ok().json(PostLoginResponse {
            authenticated: true,
        }))
    } else {
        /*
         This also says username to prevent staffing attacks
         WARNING: This is, from experience, not really useful becauase a timing attack could
                  detect the missing hash verification. I will add a fake hashing at some point
        */
        Ok(HttpResponse::Ok().json(PostLoginResponse {
            authenticated: false,
        }))
    }
}
