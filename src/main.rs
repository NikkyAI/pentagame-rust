// loading macros
#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate uuid;

// includes
mod api;
mod auth;
mod config;
mod db;
mod frontend;
// will be activated once tested and done mod logic;
mod server;

// imports
use crate::config::DEFAULT_CONFIG_NAME;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;

pub fn main() -> std::io::Result<()> {
    let matches: ArgMatches = App::new("Pentagame")
        .author("Cobalt <cobalt.rocks>")
        .version("0.0.1")
        .arg(
            Arg::with_name("config")
                .short("c")
                .default_value(DEFAULT_CONFIG_NAME)
                .long("config")
                .value_name("CONFIG")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("serve").about("serve pentagame"))
        .subcommand(
            SubCommand::with_name("generate")
                .about("generate new session key")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .default_value(config::DEFAULT_KEY_FILE)
                        .long("config")
                        .value_name("FILE")
                        .help("Set a custom output file")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // read config from 'cms.toml' and evaluate host
    let config_raw_path = match matches.value_of("config") {
        Some(path) => path,
        None => DEFAULT_CONFIG_NAME,
    }
    .to_owned();

    match matches.subcommand_matches("serve") {
        Some(_) => {
            let path_copy = config_raw_path.clone();
            server::main(path_copy)?
        }
        None => (),
    };

    match matches.subcommand_matches("generate") {
        Some(subcommand_matches) => {
            let config_path = Path::new(&config_raw_path);
            let mut config = config::Config::load_config(config_path.clone());
            config.auth.file = subcommand_matches.value_of("file").unwrap().to_owned();
            config.dump_config(&config_path)?;
            auth::generate_key(&config.auth)?;
        }
        None => (),
    };

    Ok(())
}
