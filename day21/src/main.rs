extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut shop_file = File::open(&args[1]).unwrap();
    let mut boss_file = File::open(&args[2]).unwrap();
    let mut shop_string = String::new();
    let mut boss_string = String::new();
    shop_file.read_to_string(&mut shop_string).unwrap();
    boss_file.read_to_string(&mut boss_string).unwrap();


    let mut items: Vec<Item> = Vec::new();
    let re = Regex::new(r"(\w+(?: \+\d)?)\s*(\d+)\s*(\d+)\s*(\d+)").unwrap();
    for line in shop_string.lines() {
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            items.push(Item {
                name: caps.at(1).unwrap().to_string(),
                cost: caps.at(2).unwrap().parse::<i32>().unwrap(),
                damage: caps.at(3).unwrap().parse::<i32>().unwrap(),
                armor: caps.at(4).unwrap().parse::<i32>().unwrap(),
            });
        }
    }

    for item in items {
        println!("{:?}", item);
    }

    for line in boss_string.lines() {
        println!("{}", line);
    }
}
