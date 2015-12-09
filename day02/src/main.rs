use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut total_area = 0;
    for line in input_string.lines() {
        let split = line.split('x');
        let mut v: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).collect();
        v.sort();

        // TODO This should probably be done with pattern matching
        let l = v[0];
        let w = v[1];
        let h = v[2];

        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
        let slack = l * w;
        total_area = total_area + surface_area + slack;
    }
    println!("Total area: {}", total_area);
}
