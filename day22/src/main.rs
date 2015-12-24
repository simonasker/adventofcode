use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Debug)]
struct Spell {
    name: String,
    cost: i32,
    turns: i32,
    dmg: i32,
    hp_heal: i32,
    mana_heal: i32,
    armor_boost: i32,
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

    let mut spells: Vec<Spell> = Vec::new();
    spells.push(Spell { name: "Magic Missile".to_string(), cost: 53,
    turns: 1, dmg: 4, hp_heal: 0, mana_heal: 0, armor_boost: 0 });
    spells.push(Spell { name: "Drain".to_string(), cost: 73,
    turns: 1, dmg: 2, hp_heal: 2, mana_heal: 0, armor_boost: 0 });
    spells.push(Spell { name: "Shield".to_string(), cost: 113,
    turns: 6, dmg: 0, hp_heal: 0, mana_heal: 0, armor_boost: 7 });
    spells.push(Spell { name: "Poison".to_string(), cost: 173,
    turns: 6, dmg: 3, hp_heal: 0, mana_heal: 0, armor_boost: 0 });
    spells.push(Spell { name: "Recharge".to_string(), cost: 229,
    turns: 5, dmg: 0, hp_heal: 0, mana_heal: 101, armor_boost: 0 });

    let mut player_hp = 10;
    let mut player_armor = 0;
    let mut player_mana = 250;

    let mut boss_hp = 13;
    let mut boss_damage = 8;

    let mut turn = 0;

    let mut spell_index = 0;
    let mut active_spells = [0; 5];


    loop {
        turn += 1;
        println!("{}) -- Player turn --", turn);
        println!("- Player has {} hit points, {} armor, {} mana",
            player_hp, player_armor, player_mana);
        println!("- Boss has {} hit point", boss_hp);

        // TODO since not all spells work the same way the generic spell struct
        // approach probably won't work. Try to implement this with a big match
        // clause instead. It might also be better to let one loop iteration =
        // one turn to avoid having to check things twice.
        for i in 0..5 {
            if active_spells[i] > 0 {
                active_spells[i] -= 1;
                let s = &spells[i];
            }
        }

        spell_index = 0;
        println!("Player casts {}", spells[spell_index].name);
        active_spells[spell_index] = spells[spell_index].turns;


        println!("");

        turn += 1;
        println!("{}) -- Boss turn --", turn);
        println!("- Player has {} hit points, {} armor, {} mana",
            player_hp, player_armor, player_mana);
        println!("- Boss has {} hit point", boss_hp);
        player_hp -= (boss_damage - player_armor);
        println!("Boss attacks for {} - {} = {} damage!",
            boss_damage, player_armor, (boss_damage - player_armor));

        if player_hp <= 0 {
            println!("Player dies");
            break;
        }
        println!("");
        println!("-------------------------------------------------------\n");
    }
}
