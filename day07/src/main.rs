use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Gate {
    operator: String,
    input: Vec<String>,
    output: String,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut gates: Vec<Gate> = Vec::new();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        let mut g = Gate {
            operator: String::new(),
            input: Vec::new(),
            output: String::new(),
        };

        match v.len() {
            3 => {
                g.operator = "NOOP".to_owned();
                g.input.push(v[0].to_owned());
            }
            4 => {
                g.operator = v[0].to_owned();
                g.input.push(v[1].to_owned());
            }
            5 => {
                g.operator = v[1].to_owned();
                g.input.push(v[0].to_owned());
                g.input.push(v[2].to_owned());
            },
            _ => break,
        }

        g.output = v[v.len()-1].to_owned();

        gates.push(g);
    }

    let mut wires: HashMap<String, u16> = HashMap::new();

    for (count, g) in gates.iter().enumerate() {
        println!("{}: {:?}", count, g);
        match &*g.operator {
            "NOOP" => {
                wires.insert(
                    g.output.clone(),
                    g.input[0].parse::<u16>().unwrap()
                );
            },
            "AND" => {
                let x = *wires.get(&g.input[0]).unwrap();
                let y = *wires.get(&g.input[1]).unwrap();
                wires.insert(g.output.clone(), x & y);
            },
            _ => continue,
        }
    }

    println!("{:#?}", wires);
}
