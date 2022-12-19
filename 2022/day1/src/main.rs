use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let f = File::open(config.file_path).expect("Should read input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut fattest_elf = 0;
    let mut current_elf = 0;

    while let Ok(num_bytes) = reader.read_line(&mut line) {
        // println!(
        //     "line: {}, bytes: {}, fattest: {}, current: {}",
        //     line, num_bytes, fattest_elf, current_elf
        // );

        // EOF
        if num_bytes == 0 {
            break;
        }

        if line == "\n" {
            fattest_elf = std::cmp::max(fattest_elf, current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line
            .trim()
            .parse::<i32>()
            .expect("Input should be a number convertible to i32");

        line.clear();
    }

    println!("{}", fattest_elf);
}
