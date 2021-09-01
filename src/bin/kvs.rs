#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.occurrences_of("version") {
        1 => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return;
        }
        _ => {
            println!("None")
        }
    }

    match matches.subcommand() {
        ("set", _match) => {
            eprintln!("unimplemented!");
            exit(-1);
        }
        ("get", _match) => {
            eprintln!("unimplemented!");
            exit(-1);
        }
        ("rm", _match) => {
            eprintln!("unimplemented!");
            exit(-1);
        }
        _ => unreachable!(),
    }
}
