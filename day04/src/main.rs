extern crate crypto;

use std::env;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = &args[1];

    let mut n = 1;
    loop {
        let s = format!("{}{}", input, n);

        let mut md5 = Md5::new();
        md5.input_str(&s);
        let hash = md5.result_str();

        if hash.starts_with(&args[2]) {
            break;
        }
        n += 1;
    }

    println!("Anwer: {}", n);
}
