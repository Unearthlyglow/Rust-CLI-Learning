#![allow(dead_code, unused_variables)]

// use std::{env, error::Error};
use std::error::Error;
// use std::fs;

pub struct Config {
    pub query: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an argument"),
        };

        Ok(Config { query })
    }
}

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     match
// }

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     let results = if config.ignore_case {
//         search_case_insensitive(&config.query, &contents)
//     } else {
//         search(&config.query, &contents)
//     };

//     for line in results {
//         println!("{line}");
//     }

//     Ok(())
// }
