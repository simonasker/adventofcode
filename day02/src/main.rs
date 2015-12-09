use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for line in input_string.lines() {
        let split = line.split('x');
        let mut v: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).collect();
        v.sort();

        // TODO This should probably be done with pattern matching
        let l = v[0];
        let w = v[1];
        let h = v[2];

        // Wrapping paper
        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
        let slack = l * w;
        total_paper = total_paper + surface_area + slack;

        // Ribbon
        let ribbon = 2 * l + 2 * w;
        let bow = l * w * h;
        total_ribbon = total_ribbon + ribbon + bow;
    }
    println!("Total paper area: {}", total_paper);
    println!("Total ribbon length: {}", total_ribbon);
}
