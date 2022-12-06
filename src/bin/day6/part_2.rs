use crate::part_1::read_input;

use std::collections::HashMap;

pub fn starting_point() {
    let buffer = read_input();

    let mut cur_letters: HashMap<char, i32> = HashMap::new();

    let mut repeat_count = 0;
    let mut last_letters: Vec<char> = vec![];
    for i in 0..14 {
        last_letters.insert(0, buffer[i]);
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
    for i in 14..buffer.len() {
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