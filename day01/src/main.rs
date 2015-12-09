fn main() {
    let input = "(((";

    for c in input.chars() {
        match c {
            '(' => println!("up"),
            ')' => println!("down"),
            _   => println!("INVALID"),
        }
    }
}
