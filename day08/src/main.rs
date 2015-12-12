use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut total_code_chars = 0;

    for line in input_string.lines() {
        total_code_chars += line.len();
    }

    println!("Total number of code characters: {}", total_code_chars);
}
