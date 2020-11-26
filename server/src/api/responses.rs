use crate::db::model::Game;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GetGameResponse {
    pub game: Game,
    pub users: Vec<(Uuid, String)>,
}

#[derive(Serialize)]
pub struct PostLoginResponse {
    pub authenticated: bool
}
