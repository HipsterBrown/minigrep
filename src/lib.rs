use std::env::Args;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename"),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    search(&config.query, &contents)
        .iter()
        .for_each(|line| println!("{}", line));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
