use std::fs;

pub fn starting_point() {

    let points = read_input();
    println!("You would win {points} points");
}

pub fn read_input() -> i64 {
    let contents = fs::read_to_string("src/bin/day2/input.txt")
        .expect("Couldn't open input file!");
    
    
    let points = contents.split('\n').map(
        |a| {
            let mut b = 0;
            let mut tmp = a.split(' ');
            let opponent = match tmp.next().unwrap() {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => unreachable!(),
            };
            let me = match tmp.next().unwrap() {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };
            b += me;
            if me == opponent {
                b += 3;
            } else if (me)%3+1 == opponent {
                b += 0;
            } else {
                b += 6;
            }
            b
        }
    ).sum();
   
    points
}