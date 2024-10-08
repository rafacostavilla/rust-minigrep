use std::error::Error;
use std::{env, fs};
use std::env::Args;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }

    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: Args) -> Result<Config, &'static str>{
        // Discard the first argument
        args.next();

        let query = match args.next() {
            Some(value) => value,
            _ => return Err("query not specified"),
        };

        let filename = match args.next() {
            Some(value) => value,
            _ => return Err("file not specified"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(
        Config{
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, content: &'a str)->Vec<&'a str>{
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str)->Vec<&'a str>{
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
Rust, bust, viaduct
trust
Duct";
        
        assert_eq!(vec!["Rust, bust, viaduct"], search(query, content))
    }
    #[test]
    fn case_insensitive(){
        let query = "dUcT";
        let content = "\
Rust, bust, viaduct
trust
Duct";
        
        assert_eq!(vec!["Rust, bust, viaduct","Duct"], search_case_insensitive(query, content));
    }
}