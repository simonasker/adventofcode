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

    for turn in 0.. {
        if turn % 2 == 0 {
            println!("{}) -- Player turn --", turn);
        } else {
            println!("{}) -- Boss turn --", turn);
        }

        // Apply effect of Shield
        player_armor = 0;
        if active_spells[2] > 0 {
            player_armor = 7;
        }

        println!("- Player has {} hit points, {} armor, {} mana",
            player_hp, player_armor, player_mana);
        println!("- Boss has {} hit point", boss_hp);

        // Print out Shield's timer
        if active_spells[2] > 0 {
            active_spells[2] -= 1;
            println!("Shield's timer is now {}.", active_spells[2]);
        }

        // Apply effect of Poison and print out timer
        if active_spells[3] > 0 {
            active_spells[3] -= 1;
            println!("Poison deals 3 damage; its timer is now {}.",
                active_spells[3]);
            boss_hp -= 3;
        }

        // Apply effect of Recharge and print out timer
        if active_spells[4] > 0 {
            active_spells[4] -= 1;
            println!("Recharge provides 101 mana; its timer is now {}",
                active_spells[4]);
            player_mana += 101;
        }

        if turn % 2 == 0 {  // Player turn
            spell_index = 2;

            match spell_index {
                0  if player_mana >= 53 => {
                    println!("Player casts Magic Missile, dealing 4 damage.");
                    println!("Player does 4 damage");
                    boss_hp -= 4;
                    player_mana -= 53;
                    mana_spent += 53;
                },
                1 if player_mana >= 73 => {
                    println!(
                        "Player casts Drain, dealing 2 damage, and healing 2 hit points.");
                    boss_hp -= 2;
                    player_hp += 2;
                    player_mana -= 73;
                    mana_spent += 73;
                },
                2 if player_mana >= 113 && active_spells[2] == 0 => {
                    println!("Player casts Shield");
                    active_spells[2] = 6;
                    player_mana -= 113;
                    mana_spent += 113;
                },
                3 if player_mana >= 173 && active_spells[3] == 0 => {
                    println!("Player casts Poison");
                    active_spells[3] = 6;
                    player_mana -= 173;
                    mana_spent += 173;
                },
                4 if player_mana >= 229 && active_spells[4] == 0 => {
                    println!("Player casts Recharge");
                    active_spells[4] = 5;
                    player_mana -= 229;
                    mana_spent += 229;
                },
                _ => println!("Invalid spell"),
            }

        } else {  // Boss turn
            player_hp -= (boss_damage - player_armor);
            println!("Boss attacks for {} - {} = {} damage!",
                boss_damage, player_armor, (boss_damage - player_armor));
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
