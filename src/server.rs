// imports
use crate::config::Config;
use crate::frontend::routes;
use actix_files as fs;
use actix_web::{http::ContentEncoding, middleware::Compress, web, App, HttpServer};
#[allow(unused_imports)] // Required as trait in scope for template.into_response()
use askama_actix::TemplateIntoResponse;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use sodiumoxide::init;
use std::io::Result;
use std::path::Path;

#[actix_web::main]
pub async fn main(config_raw_path: String) -> Result<()> {
    // construct path
    let config_path = Path::new(&config_raw_path);

    // load config
    let config = Config::load_config(&config_path);

    // evaluate host
    let host = match config.server.port {
        Some(port) => format!("{}:{}", config.server.ip, port),
        None => format!("{}:8080", config.server.ip),
    };

    // create database pool for app
    let manager = ConnectionManager::<PgConnection>::new(config.database.build_connspec());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // initialize actix-web server
    println!("Binding server to http://{}", &host);

    HttpServer::new(move || {
        match init() {
            Ok(_) => (),
            Err(_) => panic!("CRITICAL: Failed to initialize sodiumoxide"),
        }
        App::new()
            .data(pool.clone())
            .wrap(Compress::new(ContentEncoding::Br)) // enable brotli compression for application
            .service(
                fs::Files::new("/static", "static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .route("/", web::get().to(routes::get_index))
            .default_service(web::route().to(routes::get_error_404))
    })
    .bind(host)?
    .run()
    .await
}
