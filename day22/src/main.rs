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

    let mut mana_spent = 0;

    let mut boss_hp = 13;
    let mut boss_damage = 8;

    let mut spell_index = 0;
    let mut active_spells = [0; 5];

    // TODO Remove this. It is just to test active spell effects.
    active_spells[0] = 2;

    for turn in 0.. {
        if turn % 2 == 0 {
            println!("{}) -- Player turn --", turn);
        } else {
            println!("{}) -- Boss turn --", turn);
        }
        println!("- Player has {} hit points, {} armor, {} mana",
            player_hp, player_armor, player_mana);
        println!("- Boss has {} hit point", boss_hp);

        if turn % 2 == 0 {  // Player turn
            spell_index = 1;

            // TODO Modify the output to match the example
            match spell_index {
                0 => {
                    println!("Player casts Magic Missile");
                    println!("Player does 4 damage");
                    boss_hp -= 4;
                    player_mana -= 53;
                    mana_spent += 53;
                },
                1 => {
                    println!("Player casts Drain");
                    boss_hp -= 2;
                    player_hp += 2;
                    player_mana -= 73;
                    mana_spent += 73;
                },
                2 => {
                    println!("Player casts Shield");
                },
                3 => {
                    println!("Player casts Poison");
                },
                4 => {
                    println!("Player casts Recharge");
                },
                _ => println!("No such spell"),
            }

        } else {  // Boss turn
            player_hp -= (boss_damage - player_armor);
            println!("Boss attacks for {} - {} = {} damage!",
                boss_damage, player_armor, (boss_damage - player_armor));
        }

        for i in 0..5 {
            if active_spells[i] > 0 {
                println!("Applying effect of spell {}", i);
                active_spells[i] -= 1;
            }
        }

        if player_hp <= 0 {
            println!("Player dies");
            break;
        }
        if boss_hp <= 0 {
            println!("Boss dies. Player wins!");
            break;
        }
        println!("");
    }
    println!("Spent mana: {}", mana_spent);
}
