use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};


fn permutations<T: Clone>(n: usize, a: &mut Vec<T>, acc: &mut Vec<Vec<T>>) {
    if n == 1 {
        acc.push(a.clone());
    } else {
        for i in 0 .. n-1 {
            permutations(n-1, a, acc);
            if n % 2 == 0 {
                a.swap(i, n-1);
            } else {
                a.swap(0, n-1);
            }
        }
        permutations(n-1, a, acc);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut people = HashSet::new();
    let mut happiness_map = HashMap::new();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        people.insert(v[0]);
        let other = v[10].trim_right_matches('.');
        let pos = if v[2] == "gain" { 1 } else { -1 };
        let happiness = v[3].parse::<i32>().unwrap() * pos;
        happiness_map.insert((v[0], other), happiness);
    }

    let mut people_vec = Vec::new();
    for p in people {
        people_vec.push(p);
    }
    println!("People: {:?}", people_vec);

    let mut seatings: Vec<Vec<_>> = Vec::new();

    permutations(people_vec.len(), &mut people_vec, &mut seatings);

    println!("Number of possible seatings: {:?}", seatings.len());

    println!("{:?}", happiness_map);
}
