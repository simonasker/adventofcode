const SIZE: usize = 3;

fn index(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn to_coord(s: &str) -> Vec<usize> {
    let r: Vec<usize> = s
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    return r;
}

fn main() {
    let mut grid: Vec<u8> = vec![0; SIZE * SIZE];

    let instruction = "toggle 0,0 through 2,2";
    let v: Vec<&str> = instruction.split_whitespace().collect();

    let mut op: &str = "";
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];

    match v[0] {
        "toggle" => {
            op = "toggle";
            a = to_coord(v[1]);
            b = to_coord(v[3]);
        },
        "turn" => {
            op = v[1];
            a = to_coord(v[2]);
            b = to_coord(v[4]);
        },
        _ => println!("Invalid instruction"),
    }

    println!("{:?}, {:?}, {:?}", op, a, b);

    for x in a[0]..b[0] + 1 {
        for y in a[1]..b[1] + 1 {
            match op {
                "on"     => grid[index(x, y)] = 1,
                "off"    => grid[index(x, y)] = 0,
                "toggle" => if grid[index(x, y)] == 0 {
                    grid[index(x, y)] = 1;
                } else {
                    grid[index(x, y)] = 0;
                },
                _ => println!("Invalid instruction"),
            }
        }
    }

    // Calculate the number of lights that are turned on
    let mut total = 0;
    for i in grid {
        total += i;
    }
    println!("Total: {}", total);
}
