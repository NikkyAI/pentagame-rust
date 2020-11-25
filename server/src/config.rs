/*
Borrowed from https://gitlab.com/C0balt/oxidized-cms
*/

use crate::auth::generate_key;
use actix_web::web::ServiceConfig;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{Error, Read, Write};
use std::path::Path;
use std::process::exit;

// Types
pub type SecretKey = [u8; 4096];

// Constants for default names
pub const DEFAULT_CONFIG_NAME: &str = "pentagame.toml";
pub const DEFAULT_KEY_FILE: &str = "secret.key";

#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthenticationConfig,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct AuthenticationConfig {
    pub file: String,
    pub session: i64,
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

    pub fn init_pool(config: Config) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
        // create database pool for app
        let manager = ConnectionManager::<PgConnection>::new(config.database.build_connspec());
        Pool::builder().build(manager)
    }

    pub fn add_pool(cfg: &mut ServiceConfig) {
        cfg.data(DatabaseConfig::init_pool(CONFIG.clone()));
    }
}

impl Config {
    pub fn load_config(config_raw_path: &str) -> Config {
        let config_path = Path::new(config_raw_path);

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

    pub fn dump_config(&self, config_path: &Path) -> Result<(), Error> {
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(config_path) {
            Err(why) => panic!("couldn't create config {}: {}", config_path.display(), why),
            Ok(file) => file,
        };

        // Write the config (TOML) string to `file`, returns `io::Result<()>`
        match file.write_all(toml::to_string_pretty(self).unwrap().as_bytes()) {
            Err(why) => panic!(
                "couldn't write config to {}: {}",
                config_path.display(),
                why
            ),
            Ok(_) => println!("successfully wrote config to {}", config_path.display()),
        }

        Ok(())
    }

    pub fn create_key(&mut self, config_path: &Path) -> Result<SecretKey, Error> {
        let key_path = Path::new(DEFAULT_KEY_FILE);
        let key = generate_key(&self.auth)?;
        self.auth.file = DEFAULT_KEY_FILE.to_owned();
        self.dump_config(config_path)?;

        if key_path.exists() {
            println!(
                "Default key file '{}' exists. Overwriting key.",
                DEFAULT_KEY_FILE
            );
        }

        // creating file
        let mut key_file = match File::create(key_path) {
            Err(why) => panic!("couldn't create key {}: {}", key_path.display(), why),
            Ok(file) => file,
        };

        // Write the key (bytes) to `key_file`, returns `io::Result<()>`
        match key_file.write_all(&key) {
            Err(why) => panic!("couldn't write key to {}: {}", key_path.display(), why),
            Ok(_) => println!("successfully wrote key to {}", key_path.display()),
        }

        Ok(key)
    }

    pub fn load_key(&mut self, config_path: &Path) -> SecretKey {
        // check if new key should be generated
        if self.auth.file == "NEW" {
            println!("Auth.file was set to 'NEW' -> Generating new key");
            return self
                .create_key(config_path)
                .expect("Failed to create new secret key");
        }

        // evaluate and check path
        let key_path = Path::new(&self.auth.file);

        if !key_path.exists() {
            eprintln!("Key file doesn't exist");
            exit(1);
        } else {
            // create empty buff
            let mut key_buffer: SecretKey = [0; 4096];

            // read bytes from file
            let mut key_file = match File::open(key_path) {
                Ok(file) => file,
                Err(why) => {
                    eprintln!("couldn't read key file {}: {}", key_path.display(), why);
                    exit(1);
                }
            };

            match key_file.read_exact(&mut key_buffer) {
                Ok(_) => (),
                Err(why) => {
                    eprintln!("Failed to load key from {}: {}", key_path.display(), why);
                    exit(1);
                }
            }

            key_buffer
        }
    }
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
// Do the same for SECRET_KEY
// WARNING: This is a workaround for now and the config and key structure should be seperated
//          at some point. I will do this when I'm having too much time or getting money for this
lazy_static! {
    pub static ref CONFIG: Config = Config::load_config(DEFAULT_CONFIG_NAME);
    pub static ref SECRET_KEY: SecretKey = CONFIG.clone().load_key(Path::new(DEFAULT_KEY_FILE));
}
