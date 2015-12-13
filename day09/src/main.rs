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

    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        locations.insert(v[0]);
        locations.insert(v[2]);

        distances.insert((v[0], v[2]), v[4]);
        distances.insert((v[2], v[0]), v[4]);
    }

    let mut loc_vec = Vec::new();
    for l in locations {
        loc_vec.push(l);
    }
    println!("Locations: {:?}", loc_vec);

    let mut acc: Vec<Vec<_>> = Vec::new();

    permutations(loc_vec.len(), &mut loc_vec, &mut acc);

    println!("Number of possible routes: {:?}", acc.len());
}
