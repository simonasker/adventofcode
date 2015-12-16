use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    flight_time: i32,
    rest_time: i32,
}


fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let part = args[2].parse::<i32>().unwrap();

    let mut reindeers: Vec<Reindeer> = Vec::new();

    for line in input_string.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        reindeers.push(Reindeer{
            name: v[0].to_string(),
            speed: v[3].parse::<i32>().unwrap(),
            flight_time: v[6].parse::<i32>().unwrap(),
            rest_time: v[13].parse::<i32>().unwrap(),
        });
    }



    if part == 1 {
        let mut best_distance = 0;
        let mut best_reindeer = String::new();

        for r in reindeers {
            let mut total_time = 2503;
            let mut flight_time = r.flight_time;
            let mut distance = 0;

            while total_time > 0 {
                if flight_time > 0 {
                    total_time -= 1;
                    flight_time -= 1;
                    distance += r.speed;
                } else {
                    total_time -= r.rest_time;
                    flight_time = r.flight_time;
                }
            }

            if distance > best_distance {
                best_distance = distance;
                best_reindeer = r.name;
            }
        }

        println!("Winner: {}, {}", best_reindeer, best_distance);
    } else if part == 2 {
        let mut distances = vec![0; reindeers.len()];
        let mut score = vec![0; reindeers.len()];

        let mut flight = vec![0; reindeers.len()];
        let mut rest = vec![0; reindeers.len()];

        for (n, r) in reindeers.iter().enumerate() {
            flight[n] += r.flight_time;
            rest[n] += r.rest_time;
        }
        println!("Flight {:?}", flight);
        println!("Rest: {:?}", rest);

        for i in 0..2503 {
            for (n, r) in reindeers.iter().enumerate() {
                if flight[n] > 0 {
                    distances[n] += r.speed;
                    flight[n] -= 1;
                } else if rest[n] > 0 {
                    rest[n] -= 1;
                } else {
                    flight[n] = r.flight_time;
                    rest[n] = r.rest_time;
                    distances[n] += r.speed;
                    flight[n] -= 1;
                }
            }
        }

        println!("Distances: {:?}", distances);
        println!("Score: {:?}", score);
    }
}
