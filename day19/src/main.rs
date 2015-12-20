use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut replacements = HashMap::new();

    let mut target = String::new();
    let mut replacements_done = false;
    for line in input_string.lines() {
        if replacements_done {
            target = line.to_string();
        } else if line.len() == 0 {
            replacements_done = true;
        } else {
            let v: Vec<&str> = line.split_whitespace().collect();
            if !replacements.contains_key(v[0]) {
                let x: Vec<&str> = Vec::new();
                replacements.insert(v[0], x);
            }
            replacements.get_mut(v[0]).unwrap().push(v[2]);
        }
    }

    println!("Target: {}", target);

    let mut current_iteration = HashSet::new();
    let mut next_iteration = HashSet::new();
    let mut iterations = 0;

    let mut starting_molecule = "e".to_string();
    next_iteration.insert(starting_molecule);

    'main: loop {
        iterations += 1;

        current_iteration = next_iteration;
        next_iteration = HashSet::new();

        for mol in current_iteration {
            let mol_len = mol.len();

            for i in 0..mol_len {
                let s = &mol[i..i+1];
                if let Some(reps) = replacements.get(s) {
                    for r in reps {
                        let new_mol = format!("{}{}{}",
                            &mol[0..i], r, &mol[i+1..mol_len]);
                        if new_mol == target {
                            println!("Found target: {}", new_mol);
                            break 'main;
                        } else {
                            next_iteration.insert(new_mol);
                        }
                    }
                }
            }

            for i in 1..mol_len {
                let s = &mol[i-1..i+1];
                if let Some(reps) = replacements.get(s) {
                    for r in reps {
                        let new_mol = format!("{}{}{}",
                            &mol[0..i-1], r, &mol[i+1..mol_len]);
                        if new_mol == target {
                            println!("Found target: {}", new_mol);
                            break 'main;
                        } else {
                            next_iteration.insert(new_mol);
                        }
                    }
                }
            }
        }
    }

    println!("Iterations: {}", iterations);
}
