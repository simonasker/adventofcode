use std::collections::HashSet;

fn main() {
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
            presents += e * 10;
        }

        println!("House {} got {} presents.", house, presents);
        if presents >= 36000000 {
            break;
        }
    }
}
