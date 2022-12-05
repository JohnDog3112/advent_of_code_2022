use crate::part_1::read_input;

pub fn starting_point() {
    let (mut crates, move_instructions) = read_input();

    for instruction in move_instructions {
        let starting_index = crates[instruction.to].len();
        for _ in 0..instruction.amount {
            let tmp = crates[instruction.from].pop().unwrap();
            crates[instruction.to].insert(starting_index,tmp);
        }
    }

    //println!("{:?}", crates);

    for cr in &crates {
        print!("{}", cr[cr.len()-1]);
    }
    println!();
}