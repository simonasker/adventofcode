use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut input_string = args[1].clone();
    println!("Input: {}", input_string);

    let iterations = args[2].parse::<i32>().unwrap();

    for i in 1 .. iterations + 1 {
        println!("{}", i);
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

        input_string = String::new();
        for group in groups {
            input_string.push_str(&format!("{}{}", group.len(), group[0]));
        }
    }
    println!("Result: {}", input_string.len());
}
