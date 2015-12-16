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
    /* let mut distances = HashMap::new(); */

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        people.insert(v[0]);

        /* let distance = v[4].parse::<i32>().unwrap(); */

        /* distances.insert((v[0], v[2]), distance); */
        /* distances.insert((v[2], v[0]), distance); */
    }

    let mut people_vec = Vec::new();
    for p in people {
        people_vec.push(p);
    }
    println!("People: {:?}", people_vec);

    let mut seatings: Vec<Vec<_>> = Vec::new();

    permutations(people_vec.len(), &mut people_vec, &mut seatings);

    println!("Number of possible seatings: {:?}", seatings.len());
}
