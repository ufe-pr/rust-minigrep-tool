use std::{error::Error, fs, env};


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for result in results {
        println!("{result}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut buffer: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            buffer.push(line);
        }
    }

    buffer
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut buffer: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            buffer.push(line);
        }
    }

    buffer
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "RUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn test_search_no_result() {
        let query = "rustacean";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(Vec::<&str>::new(), search_case_sensitive(query, contents));
    }
}
