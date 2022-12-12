use std::fs;

pub fn starting_point() {
    let (starting_pos, ending_pos, grid) = read_input();

    let weight_grid = generate_weighted_grid(ending_pos.clone(), &grid);
    
    /* 
    weight_grid.iter().for_each(|line| {
        println!("{}", line.iter().map(|a| {
            format!("{:4}", a)
        }).collect::<String>());
    });*/
    println!("Fewest steps: {}", weight_grid[starting_pos.1 as usize][starting_pos.0 as usize]);


    
}

pub fn generate_weighted_grid(ending_pos: (i32, i32), grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut weight_grid: Vec<Vec<i32>> = vec![];
    for _i in 0..grid.len() {
        let mut row = vec![];
        //println!("row {i} length: {}", grid[i].len());
        for _ in 0..grid[0].len() {
            row.push(0);
        }
        weight_grid.push(row);
    }
    let mut points = vec![ending_pos];
    while points.len() > 0 {
        let mut tmp_points = vec![];
        for point in points {
            for i in 0..4 {
                let x = {if i == 0 {-1} else if i == 1 {1} else {0}};
                let y = {if i == 2 {1} else if i == 3 {-1} else {0}};
                let off_x = x + point.0;
                let off_y = y + point.1;
                if !(off_x >= 0 && off_x < grid[0].len() as i32 && off_y >= 0 && off_y < grid.len() as i32) {
                    continue;
                }
                
                //println!("{off_x} < {} = {}", grid[0].len(), off_x < grid[0].len() as i32);
                let target_point = grid[off_y as usize][off_x as usize];
                
                if target_point+1 < grid[point.1 as usize][point.0 as usize] {
                    continue;
                }
                //println!("{target_point}, {}", grid[point.1 as usize][point.0 as usize]);
                let target_val = weight_grid[off_y as usize][off_x as usize];
                let cur_val = weight_grid[point.1 as usize][point.0 as usize];

                if cur_val+1 < target_val || target_val == 0 {
                    tmp_points.push((off_x, off_y));
                    weight_grid[off_y as usize][off_x as usize] = cur_val + 1;
                }
            }
        }
        points = tmp_points;
    }
    weight_grid[ending_pos.1 as usize][ending_pos.0 as usize] = 0;

    weight_grid
}

pub fn read_input() -> ((i32, i32), (i32, i32), Vec<Vec<i32>>) {
    let contents = fs::read_to_string("src/bin/day12/input.txt")
        .expect("Couldn't open input file!");
    let mut starting_pos: (i32, i32) = (0,0);
    let mut ending_pos: (i32, i32) = (0,0);

    let mut row = -1;
    let mut col = -1;

    let grid: Vec<Vec<i32>> = contents.split("\n").map(|line| {
        row += 1;
        col = -1;
        //println!("\n");
        line.chars().filter(|a| *a as i32 != 13).map(|ch|{
            //println!("char: {ch}, {}", ch as i32);
            col += 1;
            match ch {
                'S' => {starting_pos = (col, row); 0},
                'E' => {ending_pos = (col, row); 25},
                a => a as i32 - 'a' as i32,
            }
        }).collect()
    }).collect();

    (starting_pos, ending_pos, grid)

}
