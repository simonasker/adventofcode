use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut nice = 0;
    let mut vowels = 0;
    for line in input_string.lines() {
        vowels = 0;
        for c in line.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }
            if vowels >= 3 {
                nice += 1;
                break;
            }
        }
    }
    println!("Number of nice strings: {}", nice);
}
