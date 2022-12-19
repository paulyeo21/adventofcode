use std::env;
use std::fs;

struct Config {
    file_path: string,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

fn main() {
    let args = env::args().collect();
    let config = parse_config(args);
    let contents = fs::read_to_string(config.file_path).expect("Should read input file");

    println!(contents);
}
