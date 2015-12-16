use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    flight_time: i32,
    rest_time: i32,
}


fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut reindeers: Vec<Reindeer> = Vec::new();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        reindeers.push(Reindeer{
            name: v[0].to_string(),
            speed: v[3].parse::<i32>().unwrap(),
            flight_time: v[6].parse::<i32>().unwrap(),
            rest_time: v[13].parse::<i32>().unwrap(),
        });
    }

    println!("{:?}", reindeers);
}
