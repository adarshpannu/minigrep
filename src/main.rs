// main.rs

use std::env;
use minigrep::{Config, run};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<_> = env::args().collect();
    let cfg = Config::new(&args)?;

    run(&cfg)?;

    Ok(())
}
