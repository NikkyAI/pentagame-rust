// imports
use super::schema::*;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct GameMove {
    pub id: i32,
    pub move_id: i32,
    pub game_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub host_id: Uuid,
    pub state: i16,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Move {
    pub id: i32,
    pub fnode: i16,
    pub ncounter: i16,
    pub snode: i16,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct UserGame {
    pub id: i32,
    pub player_id: Uuid,
    pub game_id: i32,
}

#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub active: bool,
    pub password: String,
    pub created_at: NaiveDateTime,
}

// Specializations

#[derive(Serialize, Deserialize)]
pub struct SlimUser {
    pub username: String,
    pub id: Uuid,
}

// Form support

#[derive(Deserialize, PartialEq, Debug)]
pub struct AuthUser {
    pub username: String,
    pub password: String,
    pub cookies: Option<String>,
}

// Insertables

#[derive(Deserialize, Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub name: String,
    pub description: Option<String>,
    pub host_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "user_games"]
pub struct NewUserGame {
    pub game_id: i32,
    pub player_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "game_moves"]
pub struct NewGameMove {
    pub game_id: i32,
    pub move_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "moves"]
pub struct NewMove {
    pub fnode: i16,
    pub ncounter: i16,
    pub snode: i16,
}

// Conversion Support
impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            username: user.username.clone(),
            id: user.id,
        }
    }
}
