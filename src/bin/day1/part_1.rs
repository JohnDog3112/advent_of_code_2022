use std::fs;

pub fn starting_point() {

    let elves = read_input();
    let mut max = 0;
    for elf in elves {
        //println!("\n{}, {}", elf.total_calories, elf.foods.len());
        if elf.total_calories > max {
            max = elf.total_calories;
        }
    }
    println!("{max}");
}

#[derive(Clone)]
pub struct Elf {
    pub foods: Vec<i32>,
    pub total_calories: i32,
}
pub fn read_input() -> Vec<Elf> {
    let contents = fs::read_to_string("src/bin/day1/input.txt")
        .expect("Couldn't open input file!");
    
    let mut elves: Vec<Elf> = vec![];
    
    let mut cur_elf = Elf {foods: vec![], total_calories: 0};
    contents.split('\n').for_each(|a| {
        if let Ok(num) = a.parse::<i32>() {
            cur_elf.total_calories += num;
            cur_elf.foods.push(num);
        } else {
            elves.push(cur_elf.clone());
            cur_elf = Elf{foods: vec![], total_calories: 0};
        }
    });
    
    elves
}