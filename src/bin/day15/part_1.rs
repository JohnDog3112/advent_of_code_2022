use std::fs;

pub fn starting_point() {

    let mut points: Vec<i32> = vec![];
    let mut collisions: Vec<i32> = vec![];
    let sensors = read_input();

    let check_line = 2000000;

    for sensor in sensors {
        let dist = (sensor.pos.1 - check_line).abs();
        //println!("{}; {}, {}", dist < sensor.distance, sensor.pos.0, sensor.pos.1);
        //println!("{}, {}", sensor.pos.0, (sensor.distance - dist));
        for i in sensor.pos.0..=(sensor.pos.0 + (sensor.distance - dist)) {
            points.push(i);
        }
        for i in  (sensor.pos.0-sensor.distance+dist)..sensor.pos.0 {
            points.push(i);
        }
        if sensor.beacon_pos.1 == check_line {
            collisions.push(sensor.beacon_pos.0);
        }
        if sensor.pos.1 == check_line {
            collisions.push(sensor.pos.0);
        }
    }
    points.sort();
    points.dedup();
    points = points.iter().filter(|a| !collisions.contains(a)).map(|a| *a).collect::<Vec<i32>>();
    //println!("{:?}", points);
    println!("{}", points.len());
}

pub struct Sensor {
    pub pos: (i32, i32),
    pub beacon_pos: (i32, i32),
    pub distance: i32,
}
pub fn read_input() -> Vec<Sensor> {
    let contents = fs::read_to_string("src/bin/day15/input.txt")
        .expect("Couldn't open input file!");
    contents.split("\n").map(|line| {
        let tmp = line.split(" ").map(|a| a.to_string()).collect::<Vec<String>>();
        let x_1 = tmp[2][2..tmp[2].len()-1].parse::<i32>().unwrap();
        let y_1 = tmp[3][2..tmp[3].len()-1].parse::<i32>().unwrap();

        let x_2 = tmp[8][2..tmp[8].len()-1].parse::<i32>().unwrap();
        let y_2 = tmp[9][2..].split("\r").next().unwrap().parse::<i32>().unwrap();
        Sensor{pos: (x_1, y_1), beacon_pos: (x_2, y_2), distance: (x_1-x_2).abs() + (y_1-y_2).abs()}
    }).collect()

}