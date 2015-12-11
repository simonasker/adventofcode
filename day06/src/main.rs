const SIZE: usize = 3;

fn index(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn main() {
    let mut grid: Vec<u8> = vec![0; SIZE * SIZE];

    println!("{:?}", grid);
    let a = (0, 0);
    let b = (2, 2);

    for x in a.0..b.0+1 {
        for y in a.1..b.1+1 {
            grid[index(x, y)] = 1;
        }
    }

    println!("{:?}", grid);
}
