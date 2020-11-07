// imports
use super::schema::*;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub host_id: Uuid,
}

#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub active: bool,
    pub password: String,
    pub created_at: NaiveDateTime,
}

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

#[derive(Serialize, Deserialize)]
pub struct SlimUser {
    pub username: String,
    pub id: Uuid,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AuthUser {
    pub username: String,
    pub password: String,
    pub cookies: Option<String>,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            username: user.username.clone(),
            id: user.id,
        }
    }
}
