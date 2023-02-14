#![warn(clippy::all, clippy::pedantic)]
use std::process;
use std::env;
use minigrep::Config;


fn main() {
    //let args: Vec<String> = env::args().collect();

    //unwrap_or_else à voir chap 13 -> closures
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}

