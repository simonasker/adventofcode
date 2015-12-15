use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let extra_string = input_string.clone();

    let mut acc = 0;
    let mut stack = Vec::new();
    // TODO This would be needed if the input didn't start with '{'
    // stack.push(0);
    let mut acc_str = String::new();
    // TODO Can read directly from input_file.chars() here
    let mut red = false;
    let mut red_depth = 0;
    for (i, c) in input_string.chars().enumerate() {
        println!("{}", stack.len());
        // println!("c: {}, stack: {:?}, acc_str: {}", c, stack, acc_str);
        if i < extra_string.len() - 6 {
            // println!("{}", &extra_string[i..i+6]);
            if &extra_string[i..i+6] == ":\"red\"" && !red {
                red = true;
                red_depth = stack.len();
                println!("RED: {}", red_depth);
            }
        }
        if c == '{' {
            stack.push(0);
            continue;
        }

        if c.is_digit(10) || c == '-' {
            acc_str.push(c);
        } else {
            if let Ok(x) = i32::from_str_radix(&acc_str, 10) {
                *stack.last_mut().unwrap() += x;
            }
            acc_str = String::new();
        }

        if c == '}' {
            let val = stack.pop().unwrap();
            if !red {
                acc += val;
            }
            if stack.len() < red_depth {
                red = false;
                red_depth = 0;
            }
        }
    }
    println!("Acc: {}", acc);
}
