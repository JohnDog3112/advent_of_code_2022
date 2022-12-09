use std::fs;

pub fn starting_point() {
    let instructions = read_input();
    let mut head_pos = Point(0,0);
    let mut last_head_pos;
    let mut tail_pos = head_pos.clone();
    let mut tail_poses = vec![Point(0,0)];
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
            last_head_pos = head_pos.clone();
            head_pos.0 += x;
            head_pos.1 += y;
            if (((head_pos.0-tail_pos.0).pow(2) + (head_pos.1-tail_pos.1).pow(2)) as f64).sqrt() > 1.5 {
                tail_poses.push(last_head_pos.clone());
                tail_pos = last_head_pos.clone();
            }
        }

    }
    tail_poses.sort();
    tail_poses.dedup();
    println!("Poses: {}", tail_poses.len());
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Point(pub i32, pub i32);
impl Point {
    pub fn break_down(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}
pub enum Movement{
    UP(i32),
    DOWN(i32),
    LEFT(i32),
    RIGHT(i32),
}
pub fn read_input() -> Vec<Movement> {
    let contents = fs::read_to_string("src/bin/day9/input.txt")
        .expect("Couldn't open input file!");
    contents.split("\n").map(|a| {
        let mut spl = a.split(" ");
        let inst = spl.next().unwrap();
        let val = spl.next().unwrap().parse::<i32>().unwrap();
        match inst {
            "U" => Movement::UP(val),
            "D" => Movement::DOWN(val),
            "L" => Movement::LEFT(val),
            "R" => Movement::RIGHT(val),
            _ => unreachable!(),
        }
    }).collect::<Vec<Movement>>()
}
