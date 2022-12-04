use crate::part_1;

pub fn starting_point() {
    let values = part_1::read_input();
    let mut total = 0;

    for i in values {
        if i[0].0 >= i[1].0 && i[0].0 <= i[1].1 {
            total += 1;
        } else if i[0].1 >= i[1].0 && i[0].0 <= i[1].1 {
            total += 1;
        }
    }
    println!("Total: {total}");
}