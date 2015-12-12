extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut total_code_chars = 0;
    let mut total_string_chars = 0;

    for line in input_string.lines() {
        total_code_chars += line.len();

        // Remove the surrounding quotes
        let mut new_line = line[1 .. line.len() - 1].to_owned();

        // Remove all escaped characters
        let re = Regex::new(r"\\(\x22|\\|x[:xdigit:]{2})").unwrap();
        new_line = re.replace_all(&new_line, "#");

        total_string_chars += new_line.len();
    }

    println!("Total number of code characters: {}", total_code_chars);
    println!("Total number of string characters: {}", total_string_chars);
    println!("Answer: {}", total_code_chars - total_string_chars);
}
