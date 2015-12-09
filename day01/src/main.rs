use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut floor = 0;
    for c in args[1].chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => continue,
        }
    }
    println!("Floor: {}", floor);
}
