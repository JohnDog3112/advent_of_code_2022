use std::fs;

pub fn starting_point() {
    let values = read_input();
    let mut total = 0;

    for i in values {
        if i[0].0 <= i[1].0 && i[0].1 >= i[1].1 {
            total += 1;
        } else if i[1].0 <= i[0].0 && i[1].1 >= i[0].1 {
            total += 1;
        }
    }
    println!("Total: {total}");
}


pub fn read_input() -> Vec<Vec<(u8, u8)>> {
    let contents = fs::read_to_string("src/bin/day4/input.txt")
        .expect("Couldn't open input file!");
    
    contents.split("\n").map(
        |a| a.split(",").map(
            |b| {
                let d: Vec<u8> = b.split("-").map(|c| c.parse::<u8>().unwrap()).collect();
                (d[0], d[1])
            }
        ).collect()
    ).collect()
}