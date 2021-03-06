extern crate regex;

use std::{env, cmp};
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

#[derive(Debug)]
struct Boss {
    hp: i32,
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

    let boss_v: Vec<&str> = boss_string.split_whitespace().collect();
    let boss = Boss {
        hp: boss_v[2].parse::<i32>().unwrap(),
        damage: boss_v[4].parse::<i32>().unwrap(),
        armor: boss_v[6].parse::<i32>().unwrap(),
    };

    let mut load_outs: Vec<[i32; 4]> = Vec::new();
    for w in 0..5 {
        for a in 5..11 {
            for r1 in 11..17 {
                for r2 in r1+1..18 {
                    // TODO handle load outs with no armor or 0 or 1 rings.
                    let load_out = [w, a, r1, r2];
                    load_outs.push(load_out);
                }
            }
        }
    }

    println!("Number of load outs: {}", load_outs.len());

    let mut lowest_winning_cost = i32::max_value();
    let mut highest_losing_cost = i32::min_value();
    for lo in load_outs {
        let mut cost = 0;
        let mut damage = 0;
        let mut armor = 0;
        for i in lo.iter() {
            let item = &items[*i as usize];
            cost += item.cost;
            damage += item.damage;
            armor += item.armor;
        }

        let mut player_hp = 100;
        let mut boss_hp = boss.hp;
        let mut player_won = false;
        loop {
            boss_hp -= cmp::max((damage - boss.armor), 1);
            if boss_hp <= 0 {
                player_won = true;
                break;
            }
            player_hp -= cmp::max((boss.damage - armor), 1);
            if player_hp <= 0 {
                break;
            }
        }
        if player_won && cost < lowest_winning_cost {
            lowest_winning_cost = cost;
        }
        if !player_won && cost > highest_losing_cost {
            highest_losing_cost = cost;
        }
    }

    println!("Lowest winning cost: {}", lowest_winning_cost);
    println!("Highest losing cost: {}", highest_losing_cost);
}
