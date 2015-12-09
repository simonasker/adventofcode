use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split('x').collect();
        let l = v[0].parse::<i32>().unwrap();
        let w = v[1].parse::<i32>().unwrap();
        let h = v[2].parse::<i32>().unwrap();
        println!("{}x{}x{}", l, w, h)
    }
}
