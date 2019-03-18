///
/// Mini grep from tutorial
/// https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
///

use std::{env, process};

use minigrep;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}




