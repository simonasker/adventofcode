use std::env;


fn increment_string(s: String) -> String {
    let mut bytes = s.into_bytes();
    let len = bytes.len();
    bytes[len - 1] += 1;
    String::from_utf8(bytes).unwrap()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_string = args[1].clone();

    println!("Input: {}", input_string);
    let inc_str = increment_string(input_string);
    println!("Output: {}", inc_str);
}
