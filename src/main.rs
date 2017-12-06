extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        println!("App error: {}", error);
        process::exit(1);
    }
}
