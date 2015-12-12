use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Gate {
    operator: String,
    output: String,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        println!("{:?}", v);

        let mut g = Gate {
            operator: String::new(),
            output: String::new(),
        };

        match v.len() {
            3 => {
                g.operator = "NOOP".to_owned();
            }
            4 => {
                g.operator = v[0].to_owned();
            }
            5 => {
                g.operator = v[1].to_owned();
            },
            _ => break,
        }

        g.output = v[v.len()-1].to_owned();

        println!("{:?}", g);
    }
}
