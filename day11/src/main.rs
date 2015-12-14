use std::env;


fn increment_string(s: String) -> String {
    let mut bytes = s.into_bytes();
    let len = bytes.len();
    for i in (0 .. bytes.len()).rev() {
        bytes[i] += 1;
        if bytes[i] >= 123 {
            bytes[i] = 97;
        } else {
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let input_string = args[1].clone();

    println!("Input: {}", input_string);
    let inc_str = increment_string(input_string);
    println!("Output: {}", inc_str);
}
