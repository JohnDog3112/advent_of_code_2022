use crate::part_1::{read_input,Point, Movement};

pub fn starting_point() {
    let instructions = read_input();
    let mut tail_poses:Vec<Point> = vec![];
    //let mut last_tail_poses: Vec<Point> = vec![];
    let mut points_traveled: Vec<Point> = vec![Point(0,0)];
    for _ in 0..10 {
        tail_poses.push(Point(0,0));
        //last_tail_poses.push(Point(0,0));
    }

    for inst in instructions {
        let (x,y, amount) = {
            let (mut x,mut y) = (0,0);
            let amount: i32;
            match inst {
                Movement::UP(val) => {y = 1; amount = val},
                Movement::DOWN(val) => {y = -1; amount = val},
                Movement::RIGHT(val) => {x = 1; amount = val},
                Movement::LEFT(val) => {x = -1; amount = val},
            }
            (x,y,amount)
        };
        for _ in 0..amount {
            //last_tail_poses[0] = tail_poses[0].clone();
            tail_poses[0].0 += x;
            tail_poses[0].1 += y;
            for i in 1..10 {
                let (x, y) = tail_poses[i-1].break_down();
                let distance = (((x-tail_poses[i].0).pow(2) + (y-tail_poses[i].1).pow(2)) as f64).sqrt();
                if distance > 1.5 {
                    //last_tail_poses[i] = tail_poses[i].clone();

                    let dist_x = x-tail_poses[i].0;
                    let dist_y = y-tail_poses[i].1;
                    if (dist_x.abs() > 1 && dist_y != 0) || (dist_y.abs() > 1 && dist_x != 0) {
                        if dist_x > 0{
                            tail_poses[i].0 += 1;
                        } else {
                            tail_poses[i].0 -= 1;
                        }
                        if dist_y > 0 {
                            tail_poses[i].1 += 1;
                        } else {
                            tail_poses[i].1 -= 1;
                        }
                    } else if dist_x > 1 {
                        tail_poses[i].0 += 1;
                    } else if dist_x < -1 {
                        tail_poses[i].0 -= 1;
                    } else if dist_y > 1 {
                        tail_poses[i].1 += 1;
                    } else if dist_y < -1 {
                        tail_poses[i].1 -= 1;
                    }

                    if i == 9 {
                        points_traveled.push(tail_poses[i].clone());
                    }
                    
                    
                }
                
            }
            //println!("\n\n");
            //print_points(&tail_poses);
            
        }
        //println!("---------------------------------------------");
        //println!("---------new line----------------------------");

    }
    points_traveled.sort();
    points_traveled.dedup();
    //print_points(&points_traveled);
    println!("Poses: {}", points_traveled.len());
    
}

#[allow(dead_code)]
pub fn print_points(points: &Vec<Point>) {
    let mut max = (0,0);
    let mut min = (0,0); //starting point
    for point in points {
        if point.0 < min.0 && point.0 <= 0{
            min.0 = point.0;
        } else if point.0 > max.0 {
            max.0 = point.0;
        }
        if point.1 < min.1 && point.1 <= 0 {
            min.1 = point.1;
        } else if point.1 > max.1 {
            max.1 = point.1;
        }
    }
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in min.1..=max.1 {
        let mut row: Vec<char> = vec![];
        for _ in min.0..=max.0 {
            row.push('.');
        }
        grid.push(row);
    }
    //0, 0
    grid[-min.1 as usize][-min.0 as usize] = 's';
    for point in points {
        grid[(point.1-min.1) as usize][(point.0-min.0) as usize] = '#';
    }
    grid.iter().rev().for_each(|a| println!("{}", a.iter().collect::<String>()));
}