use clap::{load_yaml, App};
use kvs::*;
use std::env;
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
    let app_author = &env::var("CARGO_PKG_AUTHORS").unwrap()[..];
    let app_version = &env::var("CARGO_PKG_VERSION").unwrap()[..];
    let app_description = &env::var("CARGO_PKG_DESCRIPTION").unwrap()[..];

    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml)
        .author(app_author)
        .version(app_version)
        .about(app_description);
    let matches = app.get_matches();
    let mut store = KvStore::open(current_dir()?)?;
    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            let value = matches.value_of("value").expect("value argument missing");
            store.set(key.to_string(), value.to_string())?;
        },
        ("get", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        },
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            match store.remove(key.to_string()) {
                Ok(()) => {},
                Err(KvsError::KeyNotFound()) => {
                    println!("Key not found");
                    exit(1);
                },
                Err(e) => return Err(e),
            }
        },
        _ => panic!("Invalid command!"),
    }
    Ok(())
}
