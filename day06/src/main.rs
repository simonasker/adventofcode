const SIZE: usize = 3;

fn index(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn main() {
    let mut grid: Vec<u8> = vec![0; SIZE * SIZE];

    grid[index(1, 1)] = 1;

    println!("{:?}", grid);
}
