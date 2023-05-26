//! # //! High Level Documentation For the Entire Crate.
//! //! is useful for describing **crates** and **modules**.
/// My Cratx

use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item=String>)
        -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path.")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();  // returns a boolean

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // take ownership of config. It is for this run anyway. Not required afterwards.
    let contents = fs::read_to_string(config.file_path)?;

    let results: Vec<&str>;
    if !config.ignore_case {
        results = search(&config.query, &contents);
    } else {
        results = search_case_insensitive(&config.query, &contents);
    }

    for line in results {
        println!("{line}");
    }
    Ok(())
}

// lifetime specifier required here:
// this is because we have > 1 argument and the automatic inference doesn't specify for us.
// with this, we have grouped the lifetime of 'contents' together with the returned vector.
// they'll be dropped when both goes out of scope.
// since vector is created in this function, it should go out of scope first. But,
// with lifetimes, it is effectively dropped when 'contents' is dropped.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;       // since we're inside of the inline submodule test, parent is minigrep.

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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