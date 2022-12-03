use std::fs;

pub fn starting_point() {
    let mistakes = find_similar(&read_input());

    //for i in &mistakes {
    //    println!("Mistake: {}", i);
    //}
    
    println!("Priority Sum: {}", mistakes.iter().map(|a| convert_to_number(*a)).sum::<i64>());
}

pub fn read_input() -> Vec<Vec<Vec<char>>> {
    let contents = fs::read_to_string("src/bin/day3/input.txt")
        .expect("Couldn't open input file!");
    
    let mut groups: Vec<Vec<Vec<char>>> = vec![];
    let mut group: Vec<Vec<char>> = vec![];

    for (i, st) in contents.split('\n').enumerate() {
        if i != 0 && i%3 == 0 {
            groups.push(group);
            group = vec![];
        }
        group.push(st.chars().collect::<Vec<char>>());
    }
    groups.push(group);
    
    groups
}
pub fn convert_to_number(ch: char) -> i64 {
    let tmp = ch as i64;
    if tmp > 96 {
        tmp - 96
    } else {
        tmp - 38
    }
}
pub fn find_similar(rucksacks: &Vec<Vec<Vec<char>>>) -> Vec<char> {
    rucksacks.iter().map(|j| {
        let mut a = j[0].clone();
        let mut b = j[1].clone();
        let mut c = j[2].clone();

        a.sort();
        b.sort();
        c.sort();

        a.dedup();
        //println!("a: {}", a.iter().collect::<String>());
        b.dedup();
        //println!("b: {}", b.iter().collect::<String>());
        c.dedup();
        //println!("c: {}", c.iter().collect::<String>());


        a.append(&mut b);
        a.append(&mut c);

        a.sort();

        //println!("total: {}", a.iter().collect::<String>());
        
        let mut last_char = '0';
        let mut second_last = '0';
        for ch in a {
            if ch == last_char && last_char == second_last {
                last_char = ch;
                break;
            }
            second_last = last_char;
            last_char = ch;
        }
        last_char
    }).collect()

}