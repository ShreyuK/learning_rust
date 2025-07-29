use std::{env, error::Error, fs};

// This is the main entry point for the minigrep application.
// It parses command line arguments, reads the file, and searches for the query.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// This module defines the configuration and search functionality for the minigrep application.
pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive: bool,
}

impl Config {
    // This method creates a new Config instance from command line arguments.
    // It expects at least three arguments: the program name, the query, and the filename
    // If the arguments are insufficient, it returns an error.
    // It also checks the environment variable CASE_INSENSITIVE to determine if the search should be case sensitive.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <program> <query> <filename>");
        }

        // The query and filename are cloned to avoid ownership issues.
        let query = args[1].clone();
        let filename = args[2].clone();

        // export CASE_INSENSITIVE=true to make it case insensitive
        // unset CASE_INSENSITIVE to make it case sensitive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// It includes functions to search for a query in the contents of a file,
// It returns a vector of lines that contain the query.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// This function performs a case-insensitive search for a query in the contents of a file.
// It converts both the query and each line to lowercase before checking for a match.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// It includes tests for both case-sensitive and case-insensitive searches.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
