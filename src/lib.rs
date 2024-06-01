use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

// open file
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut file = File::open(config.filename)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;
   let result = if config.case_sensitive {
       search(&config.query, &content)
   }else{
       insensitive_case_search(&config.query, &content)
   };

    for line in result{
        println!("{}", line);
    }

    Ok(())
}
// case sensitive search
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result

}
// insensitive search
pub fn insensitive_case_search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}

// tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
        Rust:\nfast, safe, productive.
        Pick three.";

        assert_eq!(vec!["fast, safe, productive."], search(query, contents));
    }
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:\nfast, safe, productive.
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:, Trust me."], insensitive_case_search(query, contents));

    }
}