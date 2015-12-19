use std::env;
use std::fs::File;
use std::io::prelude::*;

const SIZE: usize = 10;

fn print_grid(grid: &[[i32; SIZE]; SIZE]) {
    for row in grid.iter() {
        println!("{:?}", row);
    }
}

fn count_lights(grid: &[[i32; SIZE]; SIZE]) -> i32 {
    let mut lights: i32 = 0;
    for row in grid.iter() {
        for l in row.iter() {
            lights += *l;
        }
    }
    lights
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut input_file = File::open(&args[1]).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let mut grid = [[0; SIZE]; SIZE];

    for (i, l) in input_string.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                grid[i][j] = 1;
            }
        }
    }

    print_grid(&grid);

    let num_lights = count_lights(&grid);
    println!("Lights: {}", num_lights);
}
