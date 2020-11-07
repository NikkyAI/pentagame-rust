// loading macros
#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate uuid;

// includes
mod config;
mod db;
mod frontend;
mod logic;
mod server;

// imports
use crate::config::DEFAULT_CONFIG_NAME;
use clap::{App, Arg, ArgMatches, SubCommand};

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
        .get_matches();

    // read config from 'pentagame.toml' and evaluate host
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

    Ok(())
}
