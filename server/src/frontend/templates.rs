use crate::db::model::{Game, SlimUser, User};
use askama_actix::Template;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub id: Option<SlimUser>,
}

#[derive(Template)]
#[template(path = "content/rules.html")]
pub struct RulesTemplate {
    pub id: Option<SlimUser>,
}

#[derive(Template)]
#[template(path = "content/cookies.html")]
pub struct CookiesTemplate {
    pub id: Option<SlimUser>,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub message: String,
    pub code: u16,
    pub id: Option<SlimUser>,
}

#[derive(Template)]
#[template(path = "games/overview.html")]
pub struct GamesOverviewTemplate {
    pub games: Vec<(i32, String)>,
    pub id: Option<SlimUser>,
}

#[derive(Template)]
#[template(path = "games/create.html")]
pub struct GamesCreateTemplate {
    pub id: Option<SlimUser>,
    pub name: String,
    pub description: String,
    pub name_error: bool,
    pub description_error: bool,
}

#[derive(Template)]
#[template(path = "games/view.html")]
pub struct GamesViewTemplate {
    pub id: Option<SlimUser>,
    pub game: Game,
    pub is_host: bool,
    pub players: Vec<(Uuid, String)>,
}

#[derive(Template)]
#[template(path = "users/login.html")]
pub struct UserLoginTemplate {
    pub id: Option<SlimUser>,
    pub username: String,
    pub password: String,
    pub cookie_error: bool,
    pub username_error: bool,
}

#[derive(Template)]
#[template(path = "users/settings.html")]
pub struct UserSettingsTemplate {
    pub id: Option<SlimUser>,
    pub user: User,
    pub username_error: bool,
    pub password_error: bool,
    pub status_error: bool,
}

#[derive(Template)]
#[template(path = "games/game.html")]
pub struct GameBoardTemplate {
    pub id: Option<SlimUser>,
}

#[derive(Template)]
#[template(path = "users/register.html")]
pub struct UserRegisterTemplate {
    pub id: Option<SlimUser>,
    pub username: String,
    pub password: String,
    pub alert: &'static str,
    pub username_error: bool,
    pub cookie_error: bool,
    pub password_error: bool,
}

#[derive(Template)]
#[template(path = "users/view.html")]
pub struct UserViewTemplate {
    pub user: User,
    pub id: Option<SlimUser>,
}
