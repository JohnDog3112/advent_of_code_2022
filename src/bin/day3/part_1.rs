use std::fs;

pub fn starting_point() {
    let mistakes = find_similar(read_input());

    
    
    println!("Priority Sum: {}", mistakes.iter().map(|a| convert_to_number(*a)).sum::<i64>());
}

pub fn read_input() -> Vec<Vec<String>> {
    let contents = fs::read_to_string("src/bin/day3/input.txt")
        .expect("Couldn't open input file!");
    
    contents.split('\n').map(|a| vec![a[..a.len()/2].to_string(),a[a.len()/2..].to_string()]).collect()
}
pub fn convert_to_number(ch: char) -> i64 {
    let tmp = ch as i64;
    if tmp > 96 {
        tmp - 96
    } else {
        tmp - 38
    }
}
pub fn find_similar(rucksacks:Vec<Vec<String>>) -> Vec<char> {
    rucksacks.iter().map(|a| {
        let mut p1 = a[0].chars().collect::<Vec<char>>();
        let mut p2 = a[1].chars().collect::<Vec<char>>();
        p1.sort();
        p2.sort();
        p1.dedup();
        p2.dedup();
        p1.append(&mut p2);
        p1.sort();
        let mut last_char = '0';
        for ch in p1 {
            if ch == last_char {
                last_char = ch;
                break;
            }
            last_char = ch;
        }
        last_char
    }).collect()

}