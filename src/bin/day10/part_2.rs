use crate::part_1::{read_input, Instruction};

pub fn starting_point() {
    let instructions = read_input();
    let mut cur_cycle = 0;
    let mut x:i32 = 1;

    let mut screen: Vec<char> = vec![];
    for _ in 0..(40*6) {
        screen.push('.');
    }
    for instruction in instructions {
        let x_pos = cur_cycle%40;
        if x-1 <= x_pos && x_pos <= x+1 {
            screen[cur_cycle as usize] = '#';
        }
        match instruction {
            Instruction::NOOP => cur_cycle += 1,
            Instruction::ADDX(val) => {
                cur_cycle += 1;

                let x_pos = cur_cycle%40;
                if x-1 <= x_pos && x_pos <= x+1 {
                    screen[cur_cycle as usize] = '#';
                }

                cur_cycle += 1;
                
                x += val;
            }
        }
        //println!("{x}, {cur_cycle}");
    }
    let screen_string: String = screen.iter().collect();
    //println!("\n{screen_string}\n\n");
    for i in 0..6 {
        println!("{}", screen_string[(i*40)..(i*40+40)].to_string());
    }
}
