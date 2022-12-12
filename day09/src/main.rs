use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = read_file("input.txt");
    let lines = input.split("\n");

    let mut head_pos: [i32; 2] = [0, 4];
    let mut tail_pos: [i32; 2] = [0, 4];

    let mut visited: HashSet<[i32; 2]> = HashSet::new();

    for line in lines {
        if line == "" {
            continue;
        }

        println!("{line}");

        let command: Vec<&str> = line.split(" ").collect();
        let direction = command[0];
        let ammount: i32 = FromStr::from_str(command[1]).unwrap();

        for _ in 0..ammount {
            match direction {
                "L" => head_pos[0] -= 1,
                "R" => head_pos[0] += 1,
                "U" => head_pos[1] -= 1,
                "D" => head_pos[1] += 1,
                _ => {}
            }

            // Chase right
            if head_pos[0] - tail_pos[0] > 1 {
                tail_pos[0] += 1;
                if head_pos[1] - tail_pos[1] > 0 {
                    tail_pos[1] += 1;
                }
                if tail_pos[1] - head_pos[1] > 0 {
                    tail_pos[1] -= 1;
                }
            }

            // Chase left
            if tail_pos[0] - head_pos[0] > 1 {
                tail_pos[0] -= 1;
                // check diagonals
                if head_pos[1] - tail_pos[1] > 0 {
                    tail_pos[1] += 1;
                }
                if tail_pos[1] - head_pos[1] > 0 {
                    tail_pos[1] -= 1;
                }
            }

            // Chase down
            if head_pos[1] - tail_pos[1] > 1 {
                tail_pos[1] += 1;
                if head_pos[0] - tail_pos[0] > 0 {
                    tail_pos[0] += 1;
                }
                if tail_pos[0] - head_pos[0] > 0 {
                    tail_pos[0] -= 1;
                }
            }

            // Chase up
            if tail_pos[1] - head_pos[1] > 1 {
                tail_pos[1] -= 1;
                // Check diagonals
                if head_pos[0] - tail_pos[0] > 0 {
                    tail_pos[0] += 1;
                }
                if tail_pos[0] - head_pos[0] > 0 {
                    tail_pos[0] -= 1;
                }
            }

            visited.insert(tail_pos);
            println!("H: {head_pos:?}");
            println!("T: {tail_pos:?}");
        }
    }

    println!("{visited:?}");
    println!("part one: {}", visited.len());
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
