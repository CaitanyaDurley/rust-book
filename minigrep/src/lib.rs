use std::{fs, env};
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// return a vector of references to contents (not pattern)
fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut out = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
            out.push(line);
        }
    }
    out
}

pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // throw away name of program
        args.next();
        let pattern = match args.next() {
            Some(s) => s,
            None => return Err("Provide a pattern"),
        };
        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("Provide a file path"),
        };
        // check if IGNORE_CASE env var is set
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            pattern,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
This line has duct in it.
This line has Duct in it.";
        assert_eq!(vec!["This line has duct in it."], search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "Duct";
        let contents = "\
This line has duct in it.
This line has Duct in it.";
        assert_eq!(
            contents.lines().collect::<Vec<&str>>(),
            search_case_insensitive(pattern, contents)
        );
    }
}