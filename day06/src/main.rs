const SIZE: usize = 3;

fn index(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn main() {
    let mut grid: Vec<u8> = vec![0; SIZE * SIZE];

    let instruction = "toggle 0,0 through 2,2";
    let v: Vec<&str> = instruction.split_whitespace().collect();

    let mut op: &str = "";
    let mut a: &str = "";
    let mut b: &str = "";

    match v[0] {
        "toggle" => {
            op = "toggle";
            a = v[1];
            b = v[3];
        },
        "turn" => {
            op = v[1];
            a = v[2];
            b = v[4];
        },
        _ => println!("Invalid instruction"),
    }

    println!("{}, {}, {}", op, a, b);


    let a = (0, 0);
    let b = (2, 2);

    for x in a.0..b.0+1 {
        for y in a.1..b.1+1 {
            grid[index(x, y)] = 1;
        }
    }

    // Calculate the number of lights that are turned on
    let mut total = 0;
    for i in grid {
        total += i;
    }
    println!("Total: {}", total);
}
