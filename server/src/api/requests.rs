use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetGameRequest {
    // game id
    pub id: i32,
}

#[derive(Deserialize)]
pub struct PostLoginRequest {
    // username
    pub username: String,

    /*
     cookie-usage agreement. THIS IS NOT OPTIONAL
     false will result in an ValidationError
    */
    pub cookie: bool,

    // user password
    pub password: String,
}
