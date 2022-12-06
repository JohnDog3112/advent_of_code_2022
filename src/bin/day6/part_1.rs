use std::fs;
use std::collections::HashMap;

pub fn starting_point() {
    let buffer = read_input();

    let mut cur_letters: HashMap<char, i32> = HashMap::new();

    let mut repeat_count = 0;
    for i in 0..4 {
        if let Some(val) = cur_letters.get_mut(&buffer[i]) {
            *val += 1;
            repeat_count += 1;
        } else {
            cur_letters.insert(buffer[i], 1);
        }
    }

    if repeat_count == 0 {
        println!("0 iterations");
        return;
    }
    let mut last_letters: Vec<char> = vec![buffer[3], buffer[2], buffer[1], buffer[0]];
    for i in 4..buffer.len() {
        if let Some(val) = cur_letters.get_mut(&last_letters.pop().unwrap()) {
            if *val != 1 {
                repeat_count -= 1;
            }
            *val -= 1;
        }
        last_letters.insert(0, buffer[i]);
        if let Some(val) = cur_letters.get_mut(&buffer[i]) {
            *val += 1;
            if *val != 1 {
                repeat_count += 1;
            }
        } else {
            cur_letters.insert(buffer[i], 1);
        }
        if repeat_count == 0 {
            println!("{} iterations", i+1);
            break;
        }
    }

}
pub fn read_input() -> Vec<char> {
    let contents = fs::read_to_string("src/bin/day6/input.txt")
        .expect("Couldn't open input file!");
    
    let input_line = contents.split("\n").next().unwrap();

    input_line.chars().collect::<Vec<char>>()
    
}
