use std::fs;

pub fn starting_point() {
    let grid = read_input();
    let mut visibility:Vec<Vec<bool>> = vec![];
    for i in 0..grid.len() {
        visibility.push(vec![]);
        for _ in 0..grid[0].len() {
            visibility[i].push(false);
        }
    }
    let mut max:i16;
    //scans left to right
    for (row, i) in grid.iter().enumerate() {
        max = -1;
        for (col,j) in i.iter().enumerate() {
            if *j as i16 > max {
                max = *j as i16;
                visibility[row][col] = true;
            }
        }
    }
    //scans right to left
    for (row, i) in grid.iter().enumerate() {
        max = -1;
        for (col,j) in i.iter().enumerate().rev() {
            if *j as i16 > max {
                max = *j as i16;
                visibility[row][col] = true;
            }
        }
    }
    //scans top to bottom
    for col in 0..grid[0].len() {
        max = -1;
        for row in 0..grid.len() {
            if grid[row][col] as i16 > max {
                max = grid[row][col] as i16;
                visibility[row][col] = true;
            }
        }
    }
    //scans bottom to top
    for col in 0..grid[0].len() {
        max = -1;
        for row in (0..grid.len()).rev() {
            if grid[row][col] as i16 > max {
                max = grid[row][col] as i16;
                visibility[row][col] = true;
            }
        }
    }

    let mut visible = 0;
    visibility.iter().for_each(|a| {
        //println!("{:?}", a);
        a.iter().for_each(|b| {
            if *b {
                visible += 1;
            }
        })
    });

    
    println!("Visible: {visible}");
}


pub fn read_input() -> Vec<Vec<u8>> {
    let contents = fs::read_to_string("src/bin/day8/input.txt")
        .expect("Couldn't open input file!");
    
    contents.split("\n").map(|a| {
        a.chars().map(|b| b.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>()
}
