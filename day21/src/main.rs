use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut shop_file = File::open(&args[1]).unwrap();
    let mut boss_file = File::open(&args[2]).unwrap();
    let mut shop_string = String::new();
    let mut boss_string = String::new();
    shop_file.read_to_string(&mut shop_string).unwrap();
    boss_file.read_to_string(&mut boss_string).unwrap();


    for line in shop_string.lines() {
        println!("{}", line);
    }

    for line in boss_string.lines() {
        println!("{}", line);
    }
}
