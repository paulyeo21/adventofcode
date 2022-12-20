use std::env;
use std::fs::File;
use std::io::prelude::*;

use day2::{predict, score};

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let mut file = File::open(config.file_path).expect("Should read input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read to string");

    println!("{}", score(contents.clone()));
    println!("{}", predict(contents.clone()));
}
