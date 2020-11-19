// imports
use super::schema::*;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Move {
    pub id: i32,
    pub fnode: i16,
    pub ncounter: i16,
    pub snode: i16,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "game_moves"]
#[belongs_to(Move)]
#[belongs_to(Game)]
pub struct GameMove {
    pub id: i32,
    pub move_id: i32,
    pub game_id: i32,
}

/*
States:
    Player 1-5 = pid. This is order is based around the `rank` attribute of the UserGame

    - 0 (not running): Waiting for players to join
    - 1-5 (pid): Waiting for move of {pid}
    - 6-10 (pid-5): Waiting for {pid} to set stopper
    - 11-16 (10 + winner amount) (finished): ranking is changed so that winners are at the top. Winner amount is the used for
*/
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: Uuid,
    pub state: i16,
    pub public: bool,
}

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug)]
#[table_name = "user_games"]
#[belongs_to(User)]
#[belongs_to(Game)]
pub struct UserGame {
    pub id: i32,
    pub user_id: Uuid,
    pub game_id: i32,
}

#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub active: bool,
    pub password: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

// Specializations

#[derive(Serialize, Deserialize, Clone)]
pub struct SlimUser {
    pub username: String,
    pub id: Uuid,
}

// INFO: Form support moved to sever/frontend/forms

// Insertables

#[derive(Deserialize, Clone, Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub name: String,
    pub description: Option<String>,
    pub user_id: Uuid,
    pub state: i16,
    pub public: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name = "user_games"]
pub struct NewUserGame {
    pub game_id: i32,
    pub user_id: Uuid,
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
