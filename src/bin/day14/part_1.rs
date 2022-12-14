use std::fmt;
use std::fs;
use std::cmp::{min, max};

pub fn starting_point() {
    let (size, instructions) = read_input();
    let mut grid = generate_grid(&size, &instructions);
    
    let mut count = 0;
    loop {
        //println!();
        //print_grid(&grid);
        if !generate_sand(&mut grid, &size) {
            break;
        }
        count += 1;
    }
    println!("Count: {count}");
}  
fn generate_sand(grid: &mut Grid, size: &Size) -> bool {
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
            return false; //goes off to the side into the abys
        } else if matches!(grid[sand_y + 1][(sand_x - 1) as usize], Object::AIR) {
            //to the left
            sand_x -= 1;
            sand_y += 1;
        } else if sand_x+1 > (size.min().0-size.max().0) as usize  {
            return false; //goes off to the side into the abys
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
pub fn generate_grid(size: &Size, instructions: &Vec<Vec<(i32, i32)>>) -> Grid {
    let mut grid: Grid = vec![];
    for _ in 0..=size.max().1 {
        let mut row = vec![];
        for _ in size.min().0..=size.max().0 {
            row.push(Object::AIR);
        }
        grid.push(row);
    }
    for instruction in instructions {
        let mut last_point = instruction[0].clone();
        for part in instruction.iter().skip(1) {
            for x in min(last_point.0, part.0)..=max(last_point.0, part.0) {
                grid[last_point.1 as usize][(x-size.min().0) as usize] = Object::ROCK;
            }
            for y in min(last_point.1, part.1)..=max(last_point.1, part.1) {
                grid[y as usize][(last_point.0-size.min().0) as usize] = Object::ROCK;
            }
            last_point = part.clone();
        }
    }

    grid
}
pub struct Size(pub (i32, i32), pub (i32,i32));
impl Size {
    pub fn min(&self) -> (i32, i32) {
        self.0
    }
    pub fn max(&self) -> (i32, i32) {
        self.1
    }
}
pub type Grid = Vec<Vec<Object>>;

#[allow(dead_code)]
pub fn print_grid(grid: &Grid) {
    grid.iter().for_each(|a|{
        println!("{}", a.iter().map(|b| format!("{b}")).collect::<String>());
    });
}

pub enum Object {
    ROCK,
    SAND,
    AIR
}
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::AIR => ".",
            Self::ROCK => "#",
            Self::SAND => "0",
        })
    }
}
pub fn read_input() -> (Size, Vec<Vec<(i32, i32)>>) {
    let contents = fs::read_to_string("src/bin/day14/input.txt")
        .expect("Couldn't open input file!");
    let mut min_x = 1000000;
    let mut min_y = 1000000;
    let mut max_x = 0;
    let mut max_y = 0;
    let out = contents.split("\n").map(|line| {
        line.split(" -> ").map(|part| {
            let mut tmp = part.split(",");
            let x: i32 = tmp.next().unwrap().parse().unwrap();
            let y: i32 = tmp.next().unwrap().parse().unwrap();
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
            (x, y)
        }).collect::<Vec<(i32, i32)>>()
        
    }).collect::<Vec<Vec<(i32, i32)>>>();
    //println!("{min_x}, {min_y}; {max_x}, {max_y}; {}, {}", max_x-min_x, max_y-min_y);
    (Size((min_x, min_y), (max_x, max_y)), out)
}
