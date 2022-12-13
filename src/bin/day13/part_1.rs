use std::fs;
use std::cmp::min;

pub fn starting_point() {
    let pairs = read_input();
    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let state =  check_pair(&pair.0, &pair.1);
        //println!("{:?}",state);
        if let State::CORRECT = state {
            sum += i+1;
        }
    }
    println!("Correct sum: {sum}");
}
pub fn check_pair(left: &Value, right: &Value) -> State {
    match left {
        Value::CONSTANT(left_val) => {
            match right {
                Value::CONSTANT(right_val) => {
                    if left_val < right_val {
                        return State::CORRECT;
                    } else if right_val < left_val {
                        return State::INCORRECT;
                    } else {
                        return State::UNSURE;
                    }
                },
                Value::LIST(_) => {
                    return check_pair(&Value::LIST(vec![left.clone()]), right);
                },
            }
        },
        Value::LIST(left_val) => {
            match right {
                Value::CONSTANT(_) => {
                    return check_pair(left, &Value::LIST(vec![right.clone()]));
                },
                Value::LIST(right_val) => {
                    for i in 0..min(left_val.len(), right_val.len()) {
                        let tmp = check_pair(&left_val[i], &right_val[i]);
                        if let State::UNSURE = tmp {
                            continue;
                        } else {
                            return tmp;
                        }
                    }
                    if left_val.len() < right_val.len() {
                        return State::CORRECT;
                    } else if right_val.len() < left_val.len() {
                        return State::INCORRECT;
                    } else {
                        return State::UNSURE;
                    }
                },
            }
        },
    };
}

#[derive(Debug)]
pub enum State {
    UNSURE,
    INCORRECT,
    CORRECT,
}

#[derive(Debug, Clone)]
pub enum Value {
    CONSTANT(i32),
    LIST(Vec<Value>)
}
fn parse_part(part: &str) -> Value {
    let mut values: Vec<(i32, Value)> = vec![];
    let mut level = 0;
    let mut str_part = "".to_string();

    part.chars().for_each(|ch| {
        //println!("{:?}", values);
        match ch {
            '[' => {
                values.push((level, Value::LIST(vec![])));
                level += 1;
            },
            ']' => {
                if str_part != "" {
                    values.push((level, Value::CONSTANT(str_part.parse().unwrap())));
                    str_part = "".to_string();
                }
                level -= 1;
                let mut tmp = vec![];
                for i in (0..values.len()).rev() {
                    if values[i].0 > level {
                        tmp.insert(0, values.remove(i).1);
                    } else {
                        values[i].1 = Value::LIST(tmp);
                        break;
                    }
                }
            },
            ',' => {
                if str_part != "" {
                    values.push((level, Value::CONSTANT(str_part.parse().unwrap())));
                    str_part = "".to_string();
                }
            },
            a => {
                str_part.push(a);
            }
        }
    });
    //println!("{:?}", values[0].1);
    values.remove(0).1
}

pub fn read_input() -> Vec<(Value, Value)> {
    let contents = fs::read_to_string("src/bin/day13/input.txt")
        .expect("Couldn't open input file!");
    
    let mut pair1 = None;
    let mut values: Vec<(Value, Value)> = vec![];
    contents.split("\n").for_each(|part| {
        if part.len() > 2 {
            if let None = pair1 {
                pair1 = Some(parse_part(part));
            } else {
                values.push((pair1.take().unwrap(), parse_part(part)));
            }
        }
    });
    values
   

}
