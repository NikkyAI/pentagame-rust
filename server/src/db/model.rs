// imports
use super::schema::*;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/*
Gamemoves:
    Gamemoves contain information about moves (due to name conflict often called actions) done by players
    To allow actions such as placing a stopper the figure ids are used as below:
        1-25: player figures (1-5: first player with lowest UserGame.id, 6-10, second player …)
        26-36: 5 gray, 5 black stoppers
    Stoppers are 'moved' (~placed) by the player colliding with them.
    Black stoppers are then moved to a new position, where no other figure is present (validation done based on graph state)
    Gray stoppers are moved to [src.0, src.1, src.2, -1, -1, -1] to mark them as 'off board'

    When figure id == 42 -> umove: [player_figure, points, -1, -1, -1, -1]
    This is used to allow for point saving without extra column
    All other GameMoves will be removed at the end of the game
*/
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "game_moves"]
#[belongs_to(Game)]
#[belongs_to(User)]
pub struct GameMove {
    pub id: i32,
    pub game_id: i32,
    pub src: Vec<i16>,
    pub dest: Vec<i16>,
    pub user_id: Uuid,
    pub figure: i16,
}

/*
Alerts:
    Alerts are fetched over the API and should be shown as toasts (see beta notification toast in base.html)

    header_type: i16/ smallint -> Title (should be evaluated after fetching)
                -1: danger
                    ONLY TRIGGERED BY ADMIN: reserved for e.g. notification about db leak (let's hope I never need top use this)
                 0: warning || info - Notification
                    This may be a manual alert triggered by an admin
                 1: info - Update
                    Server got an update
                 2: warning - Maintenance
                    Server may go into maintenance at specified time

*/
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "alerts"]
#[belongs_to(User)]
pub struct Alert {
    pub id: i32,
    pub user_id: Uuid,
    pub header_type: i16,
    pub message: String,
}

/*
States:
    Player 1-5 = pid. This is order is based around the `rank` attribute of the UserGame

    - 0 (not running): Waiting for players to join
    - 1-5 (pid): Waiting for move of {pid}
    - 6-10 (pid-5): Waiting for {pid} to set stopper
    - 11-16 (10 + winner amount) (finished): ranking is changed so that winners are at the top. Winner amount is the used for
*/
#[derive(Identifiable, Serialize, Queryable, Associations, Clone, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: Uuid,
    pub state: i16,
    pub public: bool,
    pub icon: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "games"]
#[belongs_to(User)]
pub struct SlimGame {
    pub id: i32,
    pub user_id: Uuid,
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

#[derive(Identifiable, Insertable, Clone, Queryable, PartialEq, Debug)]
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
    pub id: Uuid,
    pub username: String,
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
    pub icon: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "user_games"]
pub struct NewUserGame {
    pub game_id: i32,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "game_moves"]
pub struct NewGameMove<'a> {
    pub game_id: i32,
    pub src: &'a [i16],  // size: 3
    pub dest: &'a [i16], // size: 3
    pub user_id: Uuid,
    pub figure: i16,
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

impl From<Game> for SlimGame {
    fn from(game: Game) -> Self {
        SlimGame {
            id: game.id,
            user_id: game.user_id,
        }
    }
}

// public constants
pub const DEFAULT_ICON: &str = "fa-hat-wizard";
pub const ICONS: [&str; 6] = [
    "fa-hand-lizard",
    "fa-chess-queen",
    "fa-chess-pawn",
    "fa-chess-rook",
    "fa-torii-gate",
    DEFAULT_ICON,
];
