use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<_> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();

    for house in 1.. {
        let mut presents = 0;

        let sq = (house as f32).sqrt() as i32;

        let mut elves = HashSet::new();
        for i in 1 .. sq + 1 {
            if house % i == 0 {
                elves.insert(i);
                elves.insert(house / i);
            }
        }

        for e in elves {
            if part == 1 {
                presents += e * 10;
            } else if part == 2 {
                if house <= e * 50 {
                    presents += e * 11;
                }
            }
        }

        println!("House {} got {} presents.", house, presents);
        if presents >= 36000000 {
            break;
        }
    }
}
