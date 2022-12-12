use crate::part_1::{read_input, generate_weighted_grid};

pub fn starting_point() {
    let (_starting_pos, ending_pos, grid) = read_input();

    let weight_grid = generate_weighted_grid(ending_pos.clone(), &grid);
    
    /* 
    weight_grid.iter().for_each(|line| {
        println!("{}", line.iter().map(|a| {
            format!("{:4}", a)
        }).collect::<String>());
    });*/

    let mut min_dist = weight_grid[0][0];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 0 {
                continue;
            }
            if min_dist > weight_grid[y][x] && weight_grid[y][x] != 0 {
                min_dist = weight_grid[y][x];
            }
        }
    }
    println!("Fewest steps: {}", min_dist);


    
}