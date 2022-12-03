use crate::part_1::read_input;

pub fn starting_point() {
    let elves = read_input();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for elf in elves {
        if elf.total_calories > c {
            c = elf.total_calories;
        }
        if elf.total_calories > b {
            c = b;
            b = elf.total_calories;
        }
        if elf.total_calories > a {
            b = a;
            a = elf.total_calories;
        }
    }
    println!("{a} + {b} + {c} = {}", a+b+c);
}