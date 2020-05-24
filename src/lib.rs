// lib.rs

pub struct Config<'a> {
    pub pattern: &'a String,
    pub filename: &'a String
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> std::result::Result<Config, &str> {
        if args.len() != 3 {
            Err("Not enough arguments")
        } else {
            let cfg = Config { pattern: &args[1], filename: &args[2] };
            Ok(cfg)
        }
    }
}

pub fn run(config: &Config) -> std::result::Result<(), std::io::Error> {
    let contents = std::fs::read_to_string(&config.filename)?;
    for s in search(&config.pattern, &contents) {
        println!("{}", s);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let v: Vec<&'a str>;
    v = contents.lines().filter(|&line| line.contains(query)).collect();
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
