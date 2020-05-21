// lib.rs

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Config {
    pub pattern: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> std::result::Result<Config, &str> {
        if args.len() != 3 {
            Err("Not enough arguments")
        } else {
            let cfg = Config { pattern: args[1].clone(), filename: args[2].clone() };
            Ok(cfg)
        }
    }
}

pub fn run(config: &Config) -> std::result::Result<(), std::io::Error> {
    let file = File::open(&config.filename)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len <= 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = vec!();

    for p in contents.split('\n') {
        if let Some(_) = p.find(query) {
            v.push(p);
        }
    }
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
