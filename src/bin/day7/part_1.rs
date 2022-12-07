use std::fs;

pub fn starting_point() {
    let directories = read_input();

    let mut total_deletion = 0;
    for i in directories {
        //println!("{}", i.size);
        if i.size <= 100000 {
            total_deletion += i.size;
        }
    }
    println!("Deletions: {total_deletion}");
}

#[derive(Debug)]
pub struct Directory {
    pub parent_id: usize,
    pub size: i64,
    pub files: Vec<File>,
    pub children: Vec<String>,
}
#[derive(Debug)]
pub struct File {
    pub size: i64,
    pub name: String,
}
pub fn read_input() -> Vec<Directory>{
    let contents = fs::read_to_string("src/bin/day7/input.txt")
        .expect("Couldn't open input file!");
    let mut lines = contents.split("\n");

    lines.next(); //remove the first line since we don't care about it
    let mut current_index = 0;
    let mut directories: Vec<Directory> = vec![];

    directories.push(Directory {
        parent_id: 0,
        size: 0,
        files: vec![],
        children: vec![],
    });

    for i in lines {
        let parameters = i.split(" ").collect::<Vec<&str>>();
        //println!("{:?}", parameters);
        if parameters[0] == "$" && parameters[1] == "cd" {
            if parameters[2] != ".." {
                directories.push(Directory{
                    parent_id: current_index, 
                    size: 0,
                    files: vec![],
                    children: vec![],
                });
                current_index = directories.len()-1;
            } else if current_index != 0 {
                let parent_index = directories[current_index].parent_id;
                directories[parent_index].size += directories[current_index].size;
                current_index = parent_index;
            }
        } else if parameters[0] == "dir" {
            directories[current_index].children.push(parameters[1].to_string());
        } else if parameters[0] != "$" {
            let file_size = parameters[0].parse::<i64>().unwrap();
            directories[current_index].size += file_size;
            directories[current_index].files.push(File {size: file_size, name: parameters[1].to_string()});
        }
    }
    let mut parent_index = current_index;
    while parent_index != 0 {
        let cur_index = parent_index;
        parent_index = directories[cur_index].parent_id;
        directories[parent_index].size += directories[cur_index].size;
    }
    //println!("{:?}", directories);
    directories
    
}
