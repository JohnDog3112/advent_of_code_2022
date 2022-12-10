use std::fs;

pub fn starting_point() {
    let instructions = read_input();
    let mut next_read = 20;
    let mut cur_cycle = 1;
    let mut x = 1;
    let mut sum = 0;
    for instruction in instructions {
        if cur_cycle == next_read {
            next_read += 40;
            sum += x*cur_cycle;
            //println!("{x} * {} = {}", cur_cycle, x*cur_cycle);
        }
        match instruction {
            Instruction::NOOP => cur_cycle += 1,
            Instruction::ADDX(val) => {
                cur_cycle += 2;
                //println!("val: {val}");
                if cur_cycle > next_read {
                    sum += x*next_read;
                    //println!("{x} * {} = {}; next: {next_read}", next_read, x*next_read);
                    next_read += 40;
                    
                }
                //println!("{val}");
                x += val;
            }
        }
    }
    println!("Total sum: {sum}");
}

pub enum Instruction{
    NOOP,
    ADDX(i32),
}
pub fn read_input() -> Vec<Instruction> {
    let contents = fs::read_to_string("src/bin/day10/input.txt")
        .expect("Couldn't open input file!");
    contents.split("\n").map(|a| {
        match a {
            "noop" => Instruction::NOOP,
            a => Instruction::ADDX(a.split(" ").last().unwrap().parse().unwrap()),
        }
    }).collect()
}
