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

fn parse_config(&args: &[String]) -> Config{
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename}

}
