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

fn parse_config(&args: &[String]) -> (&str, &str){
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
