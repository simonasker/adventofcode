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

    let part = args[2].parse::<i32>().unwrap();

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

    if part == 2 {
        wires.insert("b".to_owned(), 16076);
    }

    loop {
        for g in gates.iter() {
            if wires.contains_key(&g.output) {
                continue;
            }

            match &*g.operator {
                "NOOP" => {
                    let x = match g.input[0].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[0]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    wires.insert(g.output.clone(), x);
                },
                "AND" => {
                    let x = match g.input[0].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[0]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    let y = match g.input[1].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[1]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    wires.insert(g.output.clone(), x & y);
                },
                "OR" => {
                    let x = match g.input[0].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[0]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    let y = match g.input[1].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[1]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    wires.insert(g.output.clone(), x | y);
                },
                "NOT" => {
                    let x = match g.input[0].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[0]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    wires.insert(g.output.clone(), !x);
                },
                "RSHIFT" => {
                    let x = match g.input[0].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[0]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    let y = match g.input[1].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[1]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    wires.insert(g.output.clone(), x >> y);
                },
                "LSHIFT" => {
                    let x = match g.input[0].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[0]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    let y = match g.input[1].parse::<u16>() {
                        Ok(a) => a,
                        Err(_) => {
                            match wires.get(&g.input[1]) {
                                Some(a) => *a,
                                None => continue,
                            }
                        },
                    };
                    wires.insert(g.output.clone(), x << y);
                },
                _ => continue,
            }
        }

        if wires.contains_key("a") {
            break;
        }
    }

    println!("Signal on wire a: {}", wires.get("a").unwrap());
}
