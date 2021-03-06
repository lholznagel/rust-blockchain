#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! Terminal client application for a peer
extern crate base64;
extern crate carina_peer;
extern crate clap;
extern crate log;
extern crate loggify;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate sodiumoxide;

mod console;
mod key;

use loggify::Loggify;
use clap::{Arg, App, SubCommand};

fn main() {
    Loggify::init_with_level(log::Level::Trace).unwrap();

    let matches = App::new("Carina network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool for carina")
        .subcommand(
            SubCommand::with_name("console")
            .about("Starts the peer.")
            .arg(Arg::with_name("CONFIG")
                .value_name("config")
                .help("Sets the location of the config file.")
                .takes_value(true)
                .long("config")
                .default_value("./config.yml"))
            )
        .subcommand(
            SubCommand::with_name("genkey")
                .about("Generates a new secret key")
            )
        .subcommand(
            SubCommand::with_name("pubkey")
                .about("Generates a new public key from a secret key")
                .arg(Arg::with_name("secret key")
                    .required(true))
            )
        .get_matches();

    match matches.subcommand() {
        ("console", Some(sub_matches)) => console::execute(sub_matches),
        ("genkey", Some(sub_matches))  => key::genkey(sub_matches),
        ("pubkey", Some(sub_matches))  => key::pubkey(sub_matches),
        _                              => ()
    }
}