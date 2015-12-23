fn main() {
    for house in 1..10 {
        let mut presents = 0;
        for elf in 1..house+1 {
            if house % elf == 0 {
                presents += elf * 10;
            }
        }
        println!("House {} got {} presents.", house, presents);
    }
}
