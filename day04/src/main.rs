use std::fs;
use std::str::FromStr;

fn main() {
    let file_content = read_file("input.txt");
    let lines = file_content.split("\n");

    let mut full_overlap_count = 0;
    let mut partial_overlap_count = 0;

    for line in lines {
        if line == "" {
            break;
        }

        let mut pair = line.split(",");
        let mut elf_a = pair.next().unwrap().split("-");
        let mut elf_b = pair.next().unwrap().split("-");

        let elf_a_start: i32 = FromStr::from_str(elf_a.next().unwrap()).unwrap();
        let elf_a_end: i32 = FromStr::from_str(elf_a.next().unwrap()).unwrap();
        let elf_b_start: i32 = FromStr::from_str(elf_b.next().unwrap()).unwrap();
        let elf_b_end: i32 = FromStr::from_str(elf_b.next().unwrap()).unwrap();

        if has_full_overlap(elf_a_start, elf_a_end, elf_b_start, elf_b_end) {
            full_overlap_count += 1;
        }

        if has_partial_overlap(elf_a_start, elf_a_end, elf_b_start, elf_b_end) {
            partial_overlap_count += 1;
        }
    }

    println!("part one: {}", full_overlap_count);
    println!("part two: {}", partial_overlap_count);
}

fn has_full_overlap(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    return (a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end);
}

fn has_partial_overlap(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    return (a_start < b_start && a_end >= b_start)
        || (b_start < a_start && b_end >= a_start)
        || (a_start == b_start || a_start == b_end || b_start == a_start);
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
