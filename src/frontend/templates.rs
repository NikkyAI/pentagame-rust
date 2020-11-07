use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub message: String,
    pub code: u16,
}
