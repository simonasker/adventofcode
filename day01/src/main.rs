use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut floor = 0;
    let mut enter_basement = 0;
    for c in args[1].chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => continue,
        }
        if floor < 0 && enter_basement == 0 {
            enter_basement = 1;
            println!("ENTERED BASEMENT");
        }
    }
    println!("Floor: {}", floor);
}
