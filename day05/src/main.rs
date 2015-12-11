use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut nice = 0;
    if args[2] == "1" {
        nice = part_1(&input_string);
    } else if args[2] == "2" {
        nice = part_2(&input_string);
    }

    println!("Number of nice strings: {}", nice);
}

fn part_2(input: &String) -> i32 {
    let mut nice = 0;
    let mut repeating_single = false;

    for line in input.lines() {

        // Checks for repeating characters separated by one character
        for i in 2..line.len() {
            if line.as_bytes()[i] == line.as_bytes()[i-2] {
                repeating_single = true;
            }
        }

        if repeating_single {
            nice += 1;
        }
    }
    return nice;
}

fn part_1(input: &String) -> i32 {
    let mut nice = 0;
    let mut vowels = 0;
    let mut prev = ' ';
    let mut has_double = false;

    'lines: for line in input.lines() {
        for d in vec!["ab", "cd", "pq", "xy"] {
            if line.contains(d) {
                continue 'lines;
            }
        }
        vowels = 0;
        prev = ' ';
        has_double = false;
        for c in line.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }
            if c == prev {
                has_double = true;
            }
            prev = c;
        }
        if has_double && vowels >= 3 {
            nice += 1;
        }
    }
    return nice;
}
