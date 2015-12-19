use std::env;
use std::fs::File;
use std::io::prelude::*;

const SIZE: usize = 6;

fn print_grid(grid: &[[i32; SIZE]; SIZE]) {
    for row in grid.iter() {
        for l in row.iter() {
            match *l {
                0 => print!("."),
                1 => print!("#"),
                _ => print!("?"),
            }
        }
        print!("\n");
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

    println!("Initial state:");
    print_grid(&grid);
    println!("");

    let num_steps = 4;
    for i in 0..num_steps {
        println!("After step {}:", i);
        print_grid(&grid);
        println!("");
    }


    let num_lights = count_lights(&grid);
    println!("Lights: {}", num_lights);
}
