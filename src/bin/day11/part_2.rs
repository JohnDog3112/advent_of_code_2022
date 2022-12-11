use crate::part_1::{read_input};
pub fn starting_point() {
    let mut monkeys = read_input();

    let multiplied_divisors = {
        let mut tmp = 1;
        for monkey in &monkeys {
            tmp *= monkey.test_value;
        }
        tmp
    };
    let mut items_inspected: Vec<i128> = vec![];
    for _ in 0..monkeys.len() {
        items_inspected.push(0);
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            items_inspected[i] += monkeys[i].items.len() as i128;
            let items = monkeys[i].items.drain(..).collect::<Vec<i128>>();
            for mut item in items {
                monkeys[i].operation.apply(&mut item);
                item %= multiplied_divisors;
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