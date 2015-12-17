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

    for line in input_string.lines() {
        let re = Regex::new(
            r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)"
        ).unwrap();
        let caps = re.captures(line).unwrap();
        println!("{:?}", caps.at(0));
    }
}
