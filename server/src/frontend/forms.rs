use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameForm {
    pub name: String,
    pub public: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>
}

#[derive(Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
    pub cookies: Option<String>, // None -> False. This is optional due to some browser not sending checkbox keys when it's not checked
}

#[derive(Deserialize)]
pub struct SettingsForm {
    pub username: Option<String>,
    pub password: Option<String>,
    pub old_password: Option<String>,
    pub status: Option<String>,
}
