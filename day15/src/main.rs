use std::env;
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

fn sums(a: Vec<i32>, target: i32, n: i32, acc: &mut Vec<Vec<i32>>) {
    let mut sum: i32 = 0;
    for i in &a {
        sum += *i;
    }
    if n == 0 {
        if sum == target {
            let r = a.clone();
            acc.push(r);
        }
    } else {
        for i in 0..target-sum+1 {
            let mut a2 = a.clone();
            a2.push(i);
            sums(a2, target, n-1, acc);
        }
    }
}



fn main() {
    let args: Vec<_> = env::args().collect();
    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for line in input_string.lines() {
        // TODO Capture data with regex instead
        let v: Vec<&str> = line.split_whitespace().collect();

        ingredients.push(Ingredient{
            name: v[0].trim_right_matches(':').to_string(),
            capacity: v[2].trim_right_matches(',').parse::<i32>().unwrap(),
            durability: v[4].trim_right_matches(',').parse::<i32>().unwrap(),
            flavour: v[6].trim_right_matches(',').parse::<i32>().unwrap(),
            texture: v[8].trim_right_matches(',').parse::<i32>().unwrap(),
            calories: v[10].trim_right_matches(',').parse::<i32>().unwrap(),
        });
    }

    let mut amounts: Vec<Vec<i32>> = Vec::new();
    // sums(Vec::new(), 100, 4, &mut amounts);
    sums(Vec::new(), 100, 4, &mut amounts);

    println!("Number of combinations: {:?}", amounts.len());
}
