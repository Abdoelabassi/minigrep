use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    //let args: Vec<String> = env::args().collect();

    // read file
    let mut file = File::open("poem.txt").expect("File not found");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Something went wrong");

    println!("file with test\n{}", content);


}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(&args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}