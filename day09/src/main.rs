use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        locations.insert(v[0]);
        locations.insert(v[2]);

        distances.insert((v[0], v[2]), v[4]);
        distances.insert((v[2], v[0]), v[4]);

        println!("{:?}", v);
    }

    println!("{:?}", locations);
    println!("{:?}", locations.len());
    println!("{:?}", distances);

    println!("{:?}", distances.get(&("Tristram", "Norrath")));
    println!("{:?}", distances.get(&("Norrath", "Tristram")));
}
