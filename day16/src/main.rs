extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let part = args[2].parse::<i32>().unwrap();

    let mut known = HashMap::new();

    known.insert("children", 3);
    known.insert("cats", 7);
    known.insert("samoyeds", 2);
    known.insert("pomeranians", 3);
    known.insert("akitas", 0);
    known.insert("vizslas", 0);
    known.insert("goldfish", 5);
    known.insert("trees", 3);
    known.insert("cars", 2);
    known.insert("perfumes", 1);

    'outer: for line in input_string.lines() {
        let re = Regex::new(
            r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)"
        ).unwrap();
        let caps = re.captures(line).unwrap();

        for i in [2usize, 4usize, 6usize].iter() {
            let attr = caps.at(*i).unwrap();
            let num = caps.at(*i+1).unwrap().parse::<i32>().unwrap();

            match attr {
                "cats" | "trees" if part == 2 => {
                    if num <= *known.get(attr).unwrap() {
                        continue 'outer;
                    }
                },
                "pomeranians" | "goldfish" if part == 2 => {
                    if num >= *known.get(attr).unwrap() {
                        continue 'outer;
                    }
                },
                _ => {
                    if num != *known.get(attr).unwrap() {
                        continue 'outer;
                    }
                },
            }
        }
        println!("{:?}", caps.at(0));
        break 'outer;
    }
}
