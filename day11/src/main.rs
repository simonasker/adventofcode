use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_string = args[1].clone();
    println!("Input: {}", input_string);
}
