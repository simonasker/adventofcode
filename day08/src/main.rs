extern crate regex;

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
        println!("Line: {}", line);
        total_code_chars += line.len();

        // Remove the surrounding quotes
        let new_line = &line[1 .. line.len() - 1];
        println!("New line: {}", new_line);

        println!("");
    }

    println!("Total number of code characters: {}", total_code_chars);
}
