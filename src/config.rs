use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

// Constants for default names
pub const DEFAULT_CONFIG_NAME: &str = "pentagame.toml";

#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: Option<u32>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub database: String,
}

impl DatabaseConfig {
    pub fn build_connspec(&self) -> String {
        // evaluate if password or placeholder should be used
        let password = match &self.password {
            Some(pwd) => format!(":{}", pwd),
            None => "".to_string(),
        };

        // evaluate if default port should be sued
        let port = match &self.port {
            Some(_port) => format!(":{}", _port),
            None => ":5432".to_string(),
        };

        // build final connspec and return
        format!(
            "postgres://{}{}@{}{}/{}",
            self.user, password, self.host, port, self.database
        )
    }
}

impl Config {
    pub fn load_config(config_path: &Path) -> Config {
        if !config_path.exists() {
            println!("ERROR: config '{}' not found", config_path.display());
            exit(1)
        } else {
            let config_file = match read_to_string(config_path) {
                Ok(content) => content,
                Err(why) => {
                    println!("ERROR: unable to read '{}' {}", config_path.display(), why);
                    exit(1)
                }
            };

            match toml::from_str::<Config>(&config_file) {
                Ok(config) => config,
                Err(why) => {
                    println!(
                        "ERROR: couldn't deserialize '{}': {}",
                        config_path.display(),
                        why
                    );
                    exit(1)
                }
            }
        }
    }
}
