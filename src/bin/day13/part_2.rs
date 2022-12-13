use std::cmp::Ordering;

use crate::part_1::{read_input, State, Value};
use crate::part_1;

pub fn starting_point() {

    let mut values = {
        let pairs = read_input();
        let mut tmp = vec![];
        for pair in pairs {
            tmp.push(pair.0);
            tmp.push(pair.1);
        }
        //push dividers
        tmp.push(Value::LIST(vec![Value::LIST(vec![Value::CONSTANT(2)])]));
        tmp.push(Value::LIST(vec![Value::LIST(vec![Value::CONSTANT(6)])]));
        tmp
    };

    values.sort_by(|left, right| check_correctness(left, right));
    let mut keys = 1;
    for (i,val) in values.iter().enumerate() {
        //println!("{:?}", val);
        if check_key(val) {
            keys *= i+1;
        }
    }
    println!("Decoder Key: {keys}");


}
fn check_key(value: &Value) -> bool {
    if let Value::LIST(val1) = value {
        if val1.len() == 1 {
            if let Value::LIST(val2) = &val1[0] {
                if val2.len() == 1 {
                    if let Value::CONSTANT(val3) = val2[0] {
                        if val3 == 2 || val3 == 6 {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn check_correctness(left_part: &Value, right_part: &Value) -> Ordering {
    match part_1::check_pair(left_part, right_part) {
        State::CORRECT => Ordering::Less,
        State::INCORRECT => Ordering::Greater,
        State::UNSURE => Ordering::Equal,
    }
}