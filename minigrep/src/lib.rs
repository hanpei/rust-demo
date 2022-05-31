use std::env;
use std::error::Error;
use std::fs;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for: {}", cfg.query);
    println!("in file: {}", cfg.filename);

    let content = fs::read_to_string(cfg.filename)?;
    let results = if cfg.case_sensitive {
        search(&cfg.query, &content)
    } else {
        search_case_insensitive(&cfg.query, &content)
    };

    if results.is_empty() {
        println!("Nothing Found.")
    } else {
        for line in results {
            println!("{}", line);
        }
    }
    Ok(())
}

fn search<'a>(keyword: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for ele in content.lines() {
        if ele.contains(keyword) {
            result.push(ele);
        }
    }
    result
}

fn search_case_insensitive<'a>(keyword: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for ele in content.lines() {
        if ele.to_lowercase().contains(&keyword.to_lowercase()) {
            result.push(ele);
        }
    }
    result
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_not_found() {
        let query = "abc";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let res: Vec<&str> = Vec::new();
        assert_eq!(res, search(query, contents));
    }
}
