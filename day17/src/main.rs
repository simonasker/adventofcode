use std::env;
use std::fs::File;
use std::io::prelude::*;

fn sums(conts: Vec<i32>, acc: &mut Vec<Vec<i32>>) {
    println!("{:?}", conts);
    let length = conts.len();
    if length > 0 {
        for i in 0 .. length {
            let mut new_conts = conts.clone();
            new_conts.remove(i);
            sums(new_conts, acc);
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

    println!("{:?}", containers);
    let mut acc: Vec<Vec<i32>> = Vec::new();
    sums(containers, &mut acc);

    println!("Acc size: {}", acc.len());

}
