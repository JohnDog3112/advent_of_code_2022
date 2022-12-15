use crate::part_1::{read_input, Sensor};

pub fn starting_point() {
    let sensors = read_input();

    let mut x = 0;
    let mut y = 0;
    'outer: for i in 0..sensors.len() {
        //println!("{i}");
        for j in 0..(sensors[i].distance+1) {
            //bottom right
            x = sensors[i].pos.0 + sensors[i].distance + 1 - j;
            y = sensors[i].pos.1 - j;
            if check_possible(&sensors, x, y) {break 'outer}

            //bottom left
            x = sensors[i].pos.0 - sensors[i].distance - 1 + j;
            y = sensors[i].pos.1 - j;
            if check_possible(&sensors, x, y) {break 'outer}

            //top right
            x = sensors[i].pos.0 + sensors[i].distance + 1 - j;
            y = sensors[i].pos.1 + j;
            if check_possible(&sensors, x, y) {break 'outer}

            //top left
            x = sensors[i].pos.0 - sensors[i].distance - 1 + j;
            y = sensors[i].pos.1 + j;
            if check_possible(&sensors, x, y) {break 'outer}
        }
    }
    println!("{x}, {y}; {}", (x as i128)*4000000 + y as i128);
    
}

fn check_possible(sensors: &Vec<Sensor>, x: i32, y: i32) -> bool {
    for sensor in sensors {
        if (sensor.pos.0-x).abs() + (sensor.pos.1-y).abs() <= sensor.distance || x < 0 || y < 0 || x > 4000000 || y > 4000000{
            return false;
        }
    }
    //println!("hi");
    true
}