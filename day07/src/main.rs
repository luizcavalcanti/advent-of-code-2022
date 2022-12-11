use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    let filesystem = get_dir_sizes();

    let mut total = 0;
    for dir in filesystem.keys() {
        let dir_size = get_total(dir, &filesystem);
        if dir_size <= 100_000 {
            total += dir_size;
        }
    }
    println!("part one: {total}");

    let mut smallest_effective = 70_000_000;
    let used_space = get_total(&String::from("/"), &filesystem);
    let free_space = 70_000_000 - used_space;
    let needed_space = 30_000_000 - free_space;

    for dir in filesystem.keys() {
        let dir_size = get_total(dir, &filesystem);
        if dir_size >= needed_space && dir_size < smallest_effective {
            smallest_effective = dir_size;
        }
    }

    println!("part two: {smallest_effective}");
}

fn get_dir_sizes() -> HashMap<String, i32> {
    let input = read_file("input.txt");
    let lines = input.split("\n");
    let mut filesystem: HashMap<String, i32> = HashMap::new();

    let mut current_dir = String::new();
    for line in lines {
        if line == "" || line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        if line.starts_with("$ cd") {
            let parts: Vec<&str> = line.split(" ").collect();
            let new_dir = parts[2];

            match new_dir {
                "/" => current_dir = String::from("/"),
                ".." => {
                    let pos = current_dir.rfind('/').unwrap();
                    current_dir = String::from(current_dir.get(0..pos).unwrap());
                }
                _ => {
                    if current_dir != "/" {
                        current_dir.push_str("/");
                    }
                    current_dir.push_str(&new_dir);
                }
            }

            if current_dir == "" {
                current_dir = String::from("/");
            }
            filesystem.entry(current_dir.clone()).or_insert(0);
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            let file_size: i32 = FromStr::from_str(parts[0]).unwrap();
            filesystem
                .entry(current_dir.clone())
                .and_modify(|size| *size += file_size)
                .or_insert(file_size);
        }
    }
    return filesystem;
}

fn get_total(dir: &String, filesystem: &HashMap<String, i32>) -> i32 {
    let mut total = 0;
    for key in filesystem.keys() {
        if key.starts_with(dir) {
            total += filesystem.get(key).unwrap();
        }
    }
    return total;
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
