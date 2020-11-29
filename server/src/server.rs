use crate::api::routes as api_routes;
use crate::config::{DatabaseConfig, CONFIG, SECRET_KEY};
use crate::frontend::routes;
use crate::ws::{actor::GameServer, routes as ws_routes};
use actix::Actor;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::ContentEncoding, middleware::Compress, web, App, HttpServer};
#[allow(unused_imports)] // Required as trait in scope for template.into_response()
use askama_actix::TemplateIntoResponse;
use pentagame_logic::graph::Graph;
use sodiumoxide::init;
use std::io::Result;
use time::Duration;

#[actix_web::main]
pub async fn main() -> Result<()> {
    // Start game server actor
    let server = GameServer::default().start();

    // evaluate host
    let host = match CONFIG.server.port {
        Some(port) => format!("{}:{}", CONFIG.server.ip, port),
        None => format!("{}:8080", CONFIG.server.ip),
    };

    // get user session length
    let session_length = CONFIG.auth.session.clone();

    // clone host for server bind
    let server_bind = CONFIG.server.ip.clone();

    // db pool
    let pool = DatabaseConfig::init_pool(CONFIG.clone())
        .expect("Failed to acquire database connection pool");

    // base graph
    let g = Graph::construct_graph()
        .expect("Empty graph failed construction when starting web server actor");

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
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&SECRET_KEY.clone())
                    .name("auth")
                    .path("/")
                    .domain(server_bind.clone())
                    .max_age_time(Duration::hours(session_length))
                    .secure(false), // this can only be true if you have https
            ))
            .service(
                fs::Files::new("/static", "static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(
                web::scope("/content")
                    .route("/rules", web::get().to(routes::get_rules))
                    .route("/cookies", web::get().to(routes::get_cookies)),
            )
            .service(
                web::scope("/games")
                    .data(g.clone())
                    .service(
                        web::resource("/ws/")
                            .data(server.clone())
                            .to(ws_routes::game_route),
                    )
                    .route("/join/{id}", web::get().to(routes::get_game_join))
                    .route("/", web::get().to(routes::get_game_overview))
                    .route("/create", web::get().to(routes::get_create_game))
                    .route("/create", web::post().to(routes::post_create_game))
                    .route("/view/{id}", web::get().to(routes::get_view_game)),
            )
            .service(
                web::scope("/users")
                    .route("/login", web::get().to(routes::get_users_login))
                    .route("/login", web::post().to(routes::post_users_login))
                    .route("/logout", web::get().to(routes::get_logout_user))
                    .route("/register", web::get().to(routes::get_register_user))
                    .route("/register", web::post().to(routes::post_register_user))
                    .route("/settings", web::get().to(routes::get_settings_user)),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth").route("/login", web::post().to(api_routes::post_login)),
                    )
                    .service(
                        web::scope("/games")
                            .route("/info", web::get().to(api_routes::get_game_meta)),
                    ),
            )
            .route("/", web::get().to(routes::get_index))
            .default_service(web::route().to(routes::get_error_404))
    })
    .bind(host)?
    .run()
    .await
}
