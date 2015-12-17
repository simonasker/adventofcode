use std::env;
use std::fs::File;
use std::io::prelude::*;

const TARGET: i32 = 150;

fn sums(a: Vec<i32>, conts: Vec<i32>, acc: &mut Vec<Vec<i32>>) {
    let mut sum: i32 = 0;
    for i in &a {
        sum += *i;
    }
    if sum == TARGET {
        let r = a.clone();
        acc.push(r);
    }
    let length = conts.len();
    if length > 0 {
        for i in 0 .. length {
            let mut a1 = a.clone();
            a1.push(conts[i]);
            let mut new_conts = conts.clone();
            new_conts.split_off(i);
            sums(a1, new_conts, acc);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut containers = Vec::new();

    for line in input_string.lines() {
        containers.push(line.parse::<i32>().unwrap());
    }

    let mut acc: Vec<Vec<i32>> = Vec::new();
    sums(Vec::new(), containers, &mut acc);

    println!("Acc size: {}", acc.len());

}
