use crate::part_1::read_input;

//not exactly happy with this one....
//the rest were solved with a decent time complexity...
//this one, not so much
//I might revise this one later because I can think of better solutions

pub fn starting_point() {
    let grid = read_input();
    let mut max_score = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cur_tree = grid[row][col];
            let mut trees_above = 0;

            for i in (0..row).rev() {
                trees_above += 1;
                if grid[i][col] >= cur_tree {
                    break;
                }
            }
            let mut trees_below = 0;
            for i in (row+1)..grid.len() {
                trees_below += 1;
                if grid[i][col] >= cur_tree {
                    break;
                }
            }
            let mut trees_to_left = 0;
            for i in (0..col).rev() {
                trees_to_left += 1;
                if grid[row][i] >= cur_tree {
                    break;
                }
            }
            let mut trees_to_right = 0;
            for i in (col+1)..grid[row].len() {
                trees_to_right += 1;
                if grid[row][i] >= cur_tree {
                    break;
                }
            }
            let score = trees_to_left*trees_to_right*trees_above*trees_below;
            if max_score < score {
                max_score = score;
            }
        }
    }
    println!("Score: {max_score}");

    
}