use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let part = args[2].parse::<i32>().unwrap();

    let mut coords = HashSet::new();
    let mut current_pos = vec![(0, 0), (0, 0)];
    let mut x = 0;
    coords.insert(current_pos[x]);
    for (i, c) in input_string.chars().enumerate() {
        if part == 2 {
            x = i - (i / 2) * 2;
        }
        match c {
            '^' => current_pos[x].1 += 1,
            '>' => current_pos[x].0 += 1,
            'v' => current_pos[x].1 -= 1,
            '<' => current_pos[x].0 -= 1,
            _   => continue,
        }
        coords.insert(current_pos[x]);
    }
    println!("Number of houses visited: {}", coords.len());
}
