use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_string = &args[1];
    println!("{}", input_string);

    let mut groups: Vec<Vec<char>> = Vec::new();
    for c in input_string.chars() {
        groups.insert(0, Vec::new());
        groups[0].push(c);
    }

    println!("{:?}", groups);
}
