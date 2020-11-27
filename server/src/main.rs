// loading macros
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

// includes
mod api;
mod auth;
mod config;
mod db;
mod frontend;
mod server;
mod ws;

// imports
use crate::config::DEFAULT_CONFIG_NAME;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;

pub fn main() -> std::io::Result<()> {
    let matches: ArgMatches = App::new("pentagame online")
        .author("Cobalt <cobalt.rocks>")
        .long_about(
            "pentagame online  Copyright (C) 2020  Cobalt
        This program comes with ABSOLUTELY NO WARRANTY",
        )
        .version("0.0.1")
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
    match matches.subcommand_matches("serve") {
        Some(_) => server::main()?,
        None => (),
    };

    match matches.subcommand_matches("generate") {
        Some(subcommand_matches) => {
            let config_raw_path = match matches.value_of("config") {
                Some(path) => path,
                None => DEFAULT_CONFIG_NAME,
            }
            .to_owned();

            let config_path = Path::new(&config_raw_path);
            let mut config = config::Config::load_config(&config_raw_path);
            config.auth.file = subcommand_matches.value_of("file").unwrap().to_owned();
            config.dump_config(&config_path)?;
            auth::generate_key(&config.auth)?;
        }
        None => (),
    };

    Ok(())
}
