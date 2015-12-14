use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut acc = 0;
    let mut acc_str = String::new();
    for c in input_string.chars() {
        if c.is_digit(10) || c == '-' {
            acc_str.push(c);
        } else {
            if let Ok(x) = i32::from_str_radix(&acc_str, 10) {
                acc += x;
            }
            acc_str = String::new();
        }
    }
    println!("Acc: {}", acc);
}
