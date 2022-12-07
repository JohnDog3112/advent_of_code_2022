use crate::part_1::read_input;

pub fn starting_point() {
    let directories = read_input();

    let size_requirement = 30000000-(70000000-directories[0].size);

    //println!("{size_requirement}");
    let mut min: i64 = directories[0].size;

    for i in directories {
        if i.size >= size_requirement && i.size < min {
            min = i.size;
        }
    }
    println!("Size of deleted directory: {min}");
}