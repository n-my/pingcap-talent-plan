use clap::{load_yaml, App};
use std::env;

fn main() {
    let app_author = &env::var("CARGO_PKG_AUTHORS").unwrap()[..];
    let app_version = &env::var("CARGO_PKG_VERSION").unwrap()[..];
    let app_description = &env::var("CARGO_PKG_DESCRIPTION").unwrap()[..];

    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml)
        .author(app_author)
        .version(app_version)
        .about(app_description);
    let matches = app.get_matches();
    match matches.subcommand() {
        ("get", Some(_matches)) => panic!("unimplemented!"),
        ("set", Some(_matches)) => panic!("unimplemented!"),
        ("rm", Some(_matches)) => panic!("unimplemented!"),
        _ => panic!("Invalid command!"),
    }
}
