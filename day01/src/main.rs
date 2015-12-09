fn main() {
    let input = "(((";

    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => continue,
        }
    }
    println!("{}", floor);
}
