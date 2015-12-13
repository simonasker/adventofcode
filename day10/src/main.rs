use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_string = &args[1];
    println!("Input: {}", input_string);

    let iterations = 40;
    for i in 1 .. iterations + 1 {
        let mut groups: Vec<Vec<char>> = Vec::new();
        let mut prev: char = ' ';
        for c in input_string.chars() {
            if c != prev {
                groups.insert(0, Vec::new());
            }
            groups[0].push(c);
            prev = c;
        }
        groups.reverse();

        let mut new_input = String::new();
        for group in groups {
            new_input.push_str(&format!("{}{}", group.len(), group[0]));
        }
        println!("{:02}: {}", i, new_input);
    }
}
