use std::env;


/* fn increment_string(s: &str) -> &str { */

/* } */

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_string = args[1].clone();
    println!("Input: {}", input_string);
    println!("Bytes: {:?}", input_string.as_bytes());

    println!("Incrementing...");
    let mut bytes = input_string.into_bytes();
    let len = bytes.len();
    bytes[len - 1] += 1;
    println!("Bytes: {:?}", bytes);

    let s = String::from_utf8(bytes).unwrap();
    println!("Output: {}", s);
}
