//!
//! Mini grep from tutorial
//! https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
//!

use std::fs;
use std::env;
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    ///
    /// Create new configuration
    ///
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments"); // return error if not enough arguments
        }

        Ok(Self {
            query: args[0].clone(),
            filename: args[1].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),   // check id environment variable exist
        })
    }
}

///
/// Runs our mini-grep app
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;   // panics if can't open file

    let results = if config.case_sensitive {
        search_sensitive(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

///
/// Case sensitive search.
///
fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

///
/// Case insensitive search
///
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


//!
//! Tests
//!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
