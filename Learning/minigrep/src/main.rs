//!
//! Mini grep from tutorial
//! https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
//!
//! to run it:
//!     `cargo run <query> <file>`
//!     `CASE_INSENSITIVE cargo run <query> <filename>`
//!

use std::{env, process};

use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);    // print to stderr
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);  // print to stderr
        process::exit(1);
    }
}
