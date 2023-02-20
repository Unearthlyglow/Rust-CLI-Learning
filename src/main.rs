#![allow(dead_code, unused_variables)]

use darkla::Config;
use std::env::args;
use std::process;

fn main() {
    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("This is your number : {}", config.query);
}

// if let Err(e) = darkla::run(config) {
//     eprintln!("Application error: {e}");
//     process::exit(1);
// }

//cargo clippy
//cargo nextest run

// git init
// git add README.md
// git commit -m "first commit"
// git branch -M main
// git remote add origin https://github.com/Unearthlyglow/Rust-CLI-Learning.git
// git push -u origin main