#[allow(unused_imports)]
use crate::part_1::{read_input, generate_grid, Size, Object, print_grid, Grid};

pub fn starting_point() {
    let (size, instructions) = read_input();
    let mut size = Size(size.min(), (size.max().0, size.max().1 + 2));
    let mut grid = generate_grid(&size, &instructions);
    
    for i in 0..grid[size.max().1 as usize].len() {
        grid[size.max().1 as usize][i] = Object::ROCK;
    }
    let mut count = 0;
    loop {
        //println!();
        //print_grid(&grid);
        if !generate_sand(&mut grid, &mut size) {
            break;
        }
        count += 1;
    }
    println!("Count: {count}");
}  

fn generate_sand(grid: &mut Grid, size: &mut Size) -> bool {
    let mut sand_x: usize = (500-size.min().0) as usize;//get it local
    let mut sand_y: usize = 0;
    if !matches!(grid[sand_y][sand_x], Object::AIR) {
        return false;
    }
    loop {
        if sand_y + 1 > size.max().1 as usize{
            return false; //passed through floor, goes to abys
        } else if matches!(grid[sand_y + 1][sand_x], Object::AIR) {
            //down
            sand_y += 1;
        }  else if sand_x == 0 {
            extend_left(grid, size);
            sand_x += 1;
        } else if matches!(grid[sand_y + 1][(sand_x - 1) as usize], Object::AIR) {
            //to the left
            sand_x -= 1;
            sand_y += 1;
        } else if sand_x+1 >= (size.max().0-size.min().0) as usize  {
            extend_right(grid, size);
        } else if matches!(grid[sand_y + 1][sand_x + 1], Object::AIR) {
            //to the right
            sand_x += 1;
            sand_y += 1;
        }  else {
            //stopped
            grid[sand_y][sand_x] = Object::SAND;
            return true;
        }
    }
}

fn extend_left(grid: &mut Grid, size: &mut Size) {
    for (i,row) in grid.iter_mut().enumerate() {
        row.insert(0, if i as i32 == size.max().1 {Object::ROCK} else {Object::AIR});
    }
    *size = Size((size.min().0 - 1, size.min().1), size.max());
}
fn extend_right(grid: &mut Grid, size: &mut Size) {
    for (i,row) in grid.iter_mut().enumerate() {
        row.push(if i as i32 == size.max().1 {Object::ROCK} else {Object::AIR});
    }
    *size = Size(size.min(), (size.max().0 + 1, size.max().1));
}