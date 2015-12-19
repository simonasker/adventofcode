use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut replacements = HashMap::new();

    let mut molecule = String::new();
    let mut replacements_done = false;
    for line in input_string.lines() {
        if replacements_done {
            molecule = line.to_string();
        } else if line.len() == 0 {
            replacements_done = true;
        } else {
            println!("{}", line);
            let v: Vec<&str> = line.split_whitespace().collect();
            if !replacements.contains_key(v[0]) {
                let x: Vec<&str> = Vec::new();
                replacements.insert(v[0], x);
            }
            replacements.get_mut(v[0]).unwrap().push(v[2]);
        }
    }

    let mol_len = molecule.len();

    for i in 0..mol_len {
        let s = &molecule[i..i+1];
        println!("Substring: {}", s);
        match replacements.get(s) {
            Some(reps) => {
                for r in reps {
                    let new_mol = format!("{}{}{}",
                        &molecule[0..i], r, &molecule[i+1..mol_len]);
                    println!("New: {}", new_mol);
                }
            },
            None => println!("No match"),
        }
    }

    println!("Replacements: {:?}", replacements);
    println!("Molecule: {}", molecule);
}
