use super::errors::APIError;
use super::requests::{GetGameRequest, PostLoginRequest};
use super::responses::{GetGameResponse, PostLoginResponse};
use crate::auth::{guard_api, verify_hash};
use crate::db::actions::{get_game, get_user_by_username};
use crate::db::model::SlimUser;
use crate::frontend::routes::DbPool;
use actix_identity::Identity;
use actix_web::{web::block, web::Data, web::Json, HttpResponse};

// General Response Type
pub type APIResponse = Result<HttpResponse, APIError>;

pub async fn get_game_meta(
    pool: Data<DbPool>,
    data: Json<GetGameRequest>,
    id: Option<SlimUser>,
) -> APIResponse {
    guard_api(&id)?;
    let conn = pool.get()?;

    let game = block(move || get_game(&conn, data.id)).await?;

    if game.0.public {
        Ok(HttpResponse::Ok().json(GetGameResponse {
            game: game.0,
            users: game.1,
        }))
    } else {
        Err(APIError::ValidationError("id".to_owned()))
    }
}

pub async fn post_login(
    id: Identity,
    pool: Data<DbPool>,
    data: Json<PostLoginRequest>,
) -> APIResponse {
    // validates cookie checkbox
    if !data.0.cookie {
        return Err(APIError::ValidationError("cookie".to_owned()));
    }
    // acquiring connection from db pool
    let conn = pool.get()?;

    let sacrifice = data.username.clone();
    let result = block(move || get_user_by_username(&conn, sacrifice)).await?;

    let user = match result {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Ok().json(PostLoginResponse {
                authenticated: false,
            }));
        }
    };

    // verify hash and remember serialized SlimUser
    if verify_hash(&user.password, &data.password) {
        let user_string = serde_json::to_string(&SlimUser::from(user)).unwrap();
        id.remember(user_string);

        Ok(HttpResponse::Ok().json(PostLoginResponse {
            authenticated: true,
        }))
    } else {
        /*
         This also says username to prevent staffing attacks
         WARNING: From experience, not really useful because a timing attack could
                  detect the missing hash verification. I will add a fake hashing at some point
        */
        Ok(HttpResponse::Ok().json(PostLoginResponse {
            authenticated: false,
        }))
    }
}
