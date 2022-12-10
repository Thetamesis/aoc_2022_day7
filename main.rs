use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut path: Vec<u32> = vec![0];
    let mut sum = 0;

    let mut dir_list = vec![]; // For part 2

    for ln in &lines[1..] {
        let (a,b) = ln.split_once(' ').unwrap();
        if a == "$" {
            if b == "cd .." {
                // cd out of directory
                let end = path.len()-1; // Last index of vec
                let dir_size = path[end];

                if dir_size <= 100_000 {
                    sum += dir_size;
                }
                path.pop();
                path[end-1] += dir_size;
                dir_list.push(dir_size); // For part 2

            } else if b == "ls" {
                continue
            } else { // case of cd into dir
                path.push(0);
            }

        } else if a == "dir" {
            continue
        } else { // case of file
            let end = path.len()-1;
            path[end] += a.parse::<u32>().unwrap();
        }
    }

    println!("All relevant directories together: {sum}");

    // Part 2: Select directory for deletion

    // Empty path vec and get size of whole file system
    let mut root_size: u32 = path[0];
    for _dir in 0..path.len() {
        let end = path.len()-1; // Last index of vec
        root_size = path[end];
        path.pop();
        if path.len() != 0 {
            path[end-1] += root_size;
        }
        
        dir_list.push(root_size);
    }
    
    let must_free = root_size - 40_000_000u32;
    dir_list.sort_unstable();
    for dir in dir_list.iter() {
        if dir >= &must_free {
            println!("Size of the directory to be deleted: {dir}");
            break
        }
    }
}
