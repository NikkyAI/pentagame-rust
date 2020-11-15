use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameForm {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
    pub cookies: Option<String>, // None -> False. This is optional due to some browser not sending checkbox keys when it's not checked
}
