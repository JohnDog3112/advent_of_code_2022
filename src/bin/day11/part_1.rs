use std::fs;

pub fn starting_point() {
    let mut monkeys = read_input();

    let mut items_inspected: Vec<i128> = vec![];
    for _ in 0..monkeys.len() {
        items_inspected.push(0);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            items_inspected[i] += monkeys[i].items.len() as i128;
            let items = monkeys[i].items.drain(..).collect::<Vec<i128>>();
            for mut item in items {
                monkeys[i].operation.apply(&mut item);
                item /= 3;
                let index: usize;
                if item % monkeys[i].test_value == 0 {
                    index = monkeys[i].test_options.0 as usize;
                } else {
                    index = monkeys[i].test_options.1 as usize;
                }
                //println!("{i} -> {index}");
                monkeys[index].items.push(item);
            }
            //println!("\n");
        }
        /* 
        for i in 0..monkeys.len() {
            println!("{:?}", monkeys[i]);
        }
        println!("\n\n");*/
    }
    let mut max = 0;
    let mut second_max = 0;
    for (_i, total) in items_inspected.iter().enumerate() {
        //println!("{i}: {total}");
        
        if *total > second_max {
            second_max = *total;
        }
        if second_max > max {
            second_max = max;
            max = *total;
        }
    }
    println!("{max} * {second_max} = {}", max*second_max);
    
}


#[derive(Debug, Clone)]
pub enum Value {
    OLD,
    CONSTANT(i128),
}
impl Value {
    pub fn get(&self, old_value: i128) -> i128 {
        match self {
            Self::OLD => old_value,
            Self::CONSTANT(val) => *val,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Operation {
    ADD(Value),
    MULTIPLY(Value),
}
impl Operation {
    pub fn apply(&self, cur_value: &mut i128) {
        *cur_value = match self {
            Self::ADD(val) => *cur_value + val.get(*cur_value),
            Self::MULTIPLY(val) => *cur_value * val.get(*cur_value),
        }
    }
}
#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<i128>,
    pub operation: Operation,
    pub test_value: i128,
    pub test_options: (i128, i128),
}
pub fn read_input() -> Vec<Monkey> {
    let contents = fs::read_to_string("src/bin/day11/input.txt")
        .expect("Couldn't open input file!");
    
    let monkeys = contents.split("\n\n");

    monkeys.map(|monkey_string| {
        let mut parts = monkey_string.split("\n");
        parts.next();
        let items: Vec<i128> = 
            parts.next().unwrap().split(" ").skip(4).map(|a| {
                if a.ends_with(",") {
                    a[..a.len()-1].parse::<i128>().unwrap()
                } else {
                    a.parse::<i128>().unwrap()
                }
                
            }).collect::<Vec<i128>>();
        let operation: Operation;
        let operation_parts: Vec<String> = parts.next().unwrap().split(" ").skip(5).map(|a| a.to_string()).collect();
        let value: Value;
        match &operation_parts[2][..] {
            "old" => value = Value::OLD,
            a => value = Value::CONSTANT(a.parse().unwrap()),
        }
        match &operation_parts[1][..] {
            "*" => operation = Operation::MULTIPLY(value),
            "+" => operation = Operation::ADD(value),
            _ => unreachable!(),
        }

        let test_value: i128 = parts.next().unwrap().split(" ").last().unwrap().parse().unwrap();

        let test_option_1: i128 = parts.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let test_option_2: i128 = parts.next().unwrap().split(" ").last().unwrap().parse().unwrap();

        Monkey {items, operation, test_value, test_options: (test_option_1, test_option_2)}


    }).collect()
    
}
