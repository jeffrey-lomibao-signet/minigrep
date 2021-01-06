use std::error::Error;
use std::fs;

pub mod config;
mod search;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search::search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

