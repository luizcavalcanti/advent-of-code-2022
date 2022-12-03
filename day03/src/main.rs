use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file_content = read_file("input.txt");
    let rucksacks = file_content.split("\n");

    let mut total: i32 = 0;
    for rucksack in rucksacks {
        if rucksack.len() == 0 {
            continue;
        }
        let size = rucksack.len() / 2;
        let pocket_a = &rucksack[0..size];
        let pocket_b = &rucksack[size..];

        for c in pocket_a.chars() {
            if pocket_b.contains(c) {
                total += char_priority(c);
                break;
            }
        }
    }
    println!("part one: {}", total);
}

fn part_two() {
    let file_content = read_file("input.txt");
    let rucksacks = file_content.split("\n");

    let vec = rucksacks.collect::<Vec<&str>>();
    let groups = vec.len() / 3;

    let mut total = 0;
    for n in 0..groups {
        let first = vec[3 * n];
        let second = vec[(3 * n) + 1];
        let third = vec[(3 * n) + 2];

        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                total += char_priority(c);
                break;
            }
        }
    }
    println!("part two: {}", total);
}

fn char_priority(c: char) -> i32 {
    let mut value = c as i32;
    if value >= 97 {
        value -= 96;
    } else {
        value -= 38;
    }
    return value;
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
