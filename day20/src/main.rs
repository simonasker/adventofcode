fn main() {
    for house in 1.. {
        let mut presents = 0;
        let elf_cap = house / 2 + 1;
        for elf in 1..elf_cap {
            if house % elf == 0 {
                presents += elf * 10;
            }
        }
        presents += house * 10;
        println!("House {} got {} presents. ({})", house, presents, elf_cap);
        if presents >= 36000000 {
            break;
        }
    }
}
