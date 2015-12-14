use std::env;


fn increment_string(s: String) -> String {
    let mut bytes = s.into_bytes();
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

fn is_valid(s: &String) -> bool {
    let mut a = false;
    let bytes = s.clone().into_bytes();
    for i in 2..bytes.len() {
        if bytes[i] == bytes[i-1] + 1 && bytes[i] == bytes[i-2] + 2 {
            a = true;
        }
    }
    a
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut input_string = args[1].clone();

    println!("Input: {}", input_string);
    loop {
        input_string = increment_string(input_string);
        println!("Output: {}", input_string);
        if is_valid(&input_string) {
            break;
        }
    }
}
