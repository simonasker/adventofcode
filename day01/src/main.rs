use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut floor = 0;
    let mut enter_basement = 0;

    for (pos, c) in input_string.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => continue,
        }
        if floor < 0 && enter_basement == 0 {
            enter_basement = pos + 1;
        }
    }
    println!("Floor: {}", floor);
    println!("Entered basement at pos: {}", enter_basement);
}
