use std::fs;
use std::collections::HashSet;

fn main() {
    println!("part one: {}", first_unique_package(4));
    println!("part two: {}", first_unique_package(14));
}

fn first_unique_package(package_size: usize) -> usize {
    let buffer = read_file("input.txt");

    let buffer_size = buffer.len();
    let mut index = 0;
    for i in package_size..buffer_size {
        let mut package = buffer[i - package_size..i].chars();
        let mut package_data = HashSet::new();
        
        for p in 0..package_size {
            package_data.insert(package.next().unwrap());
        }

        if package_data.len() == package_size {
            index = i;
            break;
        }
    }
    return index;
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
