// author: EL Abassi Abderrazaq

extern crate minigrep;
use std::{env, process};

use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    // parse arguments
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Cannot parse arguments: {}", err);
        process::exit(1);
    });
    //  read file
    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }


}
