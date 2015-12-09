use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut coords = HashSet::new();
    let mut current_pos = (0, 0);
    coords.insert(current_pos);
    for c in input_string.chars() {
        match c {
            '^' => current_pos.1 += 1,
            '>' => current_pos.0 += 1,
            'v' => current_pos.1 -= 1,
            '<' => current_pos.0 -= 1,
            _   => continue,
        }
        coords.insert(current_pos);
    }

    println!("Number of houses visited: {}", coords.len());

}
