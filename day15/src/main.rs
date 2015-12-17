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
    let part = args[2].parse::<i32>().unwrap();

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
    sums(Vec::new(), 100, ingredients.len() as i32, &mut amounts);

    println!("Number of combinations: {:?}", amounts.len());

    let mut best_score = i32::min_value();
    for amount in amounts {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavour = 0;
        let mut texture = 0;
        let mut calories = 0;

        for (i, ing) in ingredients.iter().enumerate() {
            capacity += amount[i] * ing.capacity;
            durability += amount[i] * ing.durability;
            flavour += amount[i] * ing.flavour;
            texture += amount[i] * ing.texture;
            calories += amount[i] * ing.calories;
        }

        if capacity < 0 {
            capacity = 0;
        }
        if durability < 0 {
            durability = 0;
        }
        if flavour < 0 {
            flavour = 0;
        }
        if texture < 0 {
            texture = 0;
        }

        let score = capacity * durability * flavour * texture;

        if score > best_score {
            if part == 1 || calories == 500 {
                best_score = score;
            }
        }
    }

    println!("Best score: {}", best_score);
}
