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
            match tmp.next().unwrap() {
                "X" => {
                    //lose
                    if opponent == 1 {
                        b += 3;
                    } else {
                        b += opponent - 1;
                    }
                },
                "Y" => {
                    //draw
                    b += 3;
                    b += opponent;
                },
                "Z" => {
                    //win
                    b += 6;
                    b += (opponent%3)+1;
                }
                _ => unreachable!(),
            }
            b
        }
    ).sum();
   
    points
}