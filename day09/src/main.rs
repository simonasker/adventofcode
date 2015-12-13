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

    let mut a = vec![0, 1, 2];
    // let mut a = vec!["apa", "bepa", "cepa"];
    // println!("Before: {:?}", a);
    let mut acc: Vec<Vec<_>> = Vec::new();

    permutations(a.len(), &mut a, &mut acc);

    println!("{:?}", acc.len());

    /* let perms = permutations(&mut a); */
    /* println!("After: {:?}", perms); */

}
