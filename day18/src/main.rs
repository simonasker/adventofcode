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

fn num_neighbors(grid: &[[i32; SIZE]; SIZE], i: usize, j: usize) -> i32 {
    let mut result = 0;
    // Check the top row
    if i > 0 {
        result += grid[i-1][j];
        if j > 0 {
            result += grid[i-1][j-1];
        }
        if j < SIZE-1 {
            result += grid[i-1][j+1];
        }
    }
    // Check the bottom row
    if i < SIZE-1 {
        result += grid[i+1][j];
        if j > 0 {
            result += grid[i+1][j-1];
        }
        if j < SIZE-1 {
            result += grid[i+1][j+1];
        }
    }
    // Check left
    if j > 0 {
        result += grid[i][j-1];
    }
    // Check right
    if j < SIZE-1 {
        result += grid[i][j+1];
    }
    result
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
    println!("Neighbors (4, 4): {}", num_neighbors(&grid, 4, 4));
    println!("Neighbors (0, 0): {}", num_neighbors(&grid, 0, 0));
    println!("Neighbors (0, 9): {}", num_neighbors(&grid, 0, 9));
    println!("Neighbors (9, 0): {}", num_neighbors(&grid, 9, 0));
    println!("Neighbors (9, 9): {}", num_neighbors(&grid, 9, 9));
}
