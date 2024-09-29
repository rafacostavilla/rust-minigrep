use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    
    // println!("With text:\n");
    // println!("----------------------------");
    // println!("{contents}");
    // println!("----------------------------");

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
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

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
    let mut result = vec![];
    
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

    result
}
fn search_case_insensitive<'a>(query: &str, content: &'a str)->Vec<&'a str>{
    let mut result = vec![];
    let query = query.to_lowercase();
    
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn search_for_case_sensitive(){
        let content = "\
Rust, bust, viaduct
trust
Duct";
        
        assert_eq!(vec!["Rust, bust, viaduct"], search("duct", content))
    }
    #[test]
    fn search_for_case_insensitive(){
        let content = "\
Rust, bust, viaduct
trust
Duct";
        
        assert_eq!(vec!["Rust, bust, viaduct","Duct"], search_case_insensitive("duct", content));
    }
}