use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Boss {
    hp: i32,
    damage: i32,
}


fn main() {
    let args: Vec<_> = env::args().collect();

    let mut boss_file = File::open(&args[1]).unwrap();
    let mut boss_string = String::new();
    boss_file.read_to_string(&mut boss_string).unwrap();

    let boss_v: Vec<&str> = boss_string.split_whitespace().collect();
    let boss = Boss {
        hp: boss_v[2].parse::<i32>().unwrap(),
        damage: boss_v[4].parse::<i32>().unwrap(),
    };

    let mut player_hp = 10;
    let mut player_armor = 0;
    let mut player_mana = 250;

    let mut boss_hp = 13;
    let mut boss_damage = 8;

    let mut turn = 0;

    loop {
        turn += 1;
        println!("{}) -- Player turn --", turn);
        println!("- Player has {} hit points, {} armor, {} mana",
            player_hp, player_armor, player_mana);
        println!("- Boss has {} hit point", boss_hp);
        println!("ATTACKS AND EFFECTS");
        println!("");

        turn += 1;
        println!("{}) -- Boss turn --", turn);
        println!("- Player has {} hit points, {} armor, {} mana",
            player_hp, player_armor, player_mana);
        println!("- Boss has {} hit point", boss_hp);
        println!("ATTACKS AND EFFECTS");
        println!("");


        if turn >= 10 {
            break;
        }
    }
}
