use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("You entered: {}", args[1]);
    let input = &args[1];

    let mut n = 1;
    loop {
        let s = format!("{}{}", input, n);
        println!("{}", s);
        if n == 10 {
            break;
        }
        n += 1;
    }

    println!("Anwer: {}", n);
}
