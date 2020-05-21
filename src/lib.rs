// lib.rs

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Config {
    pub pattern: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> std::result::Result<Config, &'static str> {
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
