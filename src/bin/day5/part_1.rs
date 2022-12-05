use std::fs;

pub fn starting_point() {

    let (mut crates, move_instructions) = read_input();

    for instruction in move_instructions {
        for _ in 0..instruction.amount {
            let tmp = crates[instruction.from].pop().unwrap();
            crates[instruction.to].push(tmp);
        }
    }

    //println!("{:?}", crates);

    for cr in &crates {
        print!("{}", cr[cr.len()-1]);
    }
    println!();

}


pub struct Move {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}
pub fn read_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let contents = fs::read_to_string("src/bin/day5/input.txt")
        .expect("Couldn't open input file!");
    
    

    let mut sections = contents.split("\n\n");

    let crate_section = sections.next().unwrap();
    let move_section = sections.next().unwrap();
    
    let crate_lines = crate_section.split("\n").map(|a| a.chars());
    
    let mut crate_stack: Vec<Vec<char>> = vec![];
    
    'loop1: for line in crate_lines {
        for (i, ch) in line.enumerate() {
            if ch == '1' {
                break 'loop1;
            }
            if ch != ' ' && ch != '[' && ch != ']' {
                let index = (i-1)/4;
                if index+1 > crate_stack.len() {
                    for _ in crate_stack.len()..=index {
                        crate_stack.push(vec![]);
                    }
                }
                crate_stack[index].insert(0,ch);
            }
        }
    }
    //println!("{:?}", crate_stack);

    let move_instructions = move_section.split('\n').map(|a| {
        let parts = a.split(' ').collect::<Vec<&str>>();
        Move {
            amount: parts[1].parse().unwrap(),
            from: parts[3].parse::<usize>().unwrap()-1,
            to: parts[5].parse::<usize>().unwrap()-1,
        }
    }).collect::<Vec<Move>>();

    (crate_stack, move_instructions)
    
    
}
