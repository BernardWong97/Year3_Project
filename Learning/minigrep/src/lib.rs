///
/// Mini grep from tutorial
/// https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
///

use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        Ok(Self {
            query: args[0].clone(),
            filename: args[1].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    println!("Arguments: {:?}", config);
    println!("With text:\n{}", contents);

    Ok(())
}