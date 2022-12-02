use std::fs;

fn main() {
    let file_contents = read_file("input.txt");
    let lines = file_contents.split("\n");

    let mut all_elves = Vec::new();
    let mut total = 0;
    for line in lines {
        if line != "" {
            total += line.parse::<i32>().unwrap();
        } else {
            all_elves.push(total);
            total = 0;
        }
    }

    // reverse sorting
    all_elves.sort_by(|a, b| b.cmp(a));

    let best_three = all_elves[0] + all_elves[1] + all_elves[2];

    println!("part one: {}", all_elves[0]);
    println!("part two: {}", best_three);
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
