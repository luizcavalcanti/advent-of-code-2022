use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = read_file("input.txt");
    let lines = input.split("\n");

    let mut nodes: [[i32; 2]; 10] = [
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
        [0, 4],
    ];

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
                "L" => nodes[0][0] -= 1,
                "R" => nodes[0][0] += 1,
                "U" => nodes[0][1] -= 1,
                "D" => nodes[0][1] += 1,
                _ => {}
            }

            for i in 1..nodes.len() {
                // Chase right
                if nodes[i-1][0] - nodes[i][0] > 1 {
                    nodes[i][0] += 1;
                    if nodes[i-1][1] - nodes[i][1] > 0 {
                        nodes[i][1] += 1;
                    }
                    if nodes[i][1] - nodes[i-1][1] > 0 {
                        nodes[i][1] -= 1;
                    }
                }

                // Chase left
                if nodes[i][0] - nodes[i-1][0] > 1 {
                    nodes[i][0] -= 1;
                    // check diagonals
                    if nodes[i-1][1] - nodes[i][1] > 0 {
                        nodes[i][1] += 1;
                    }
                    if nodes[i][1] - nodes[i-1][1] > 0 {
                        nodes[i][1] -= 1;
                    }
                }

                // Chase down
                if nodes[i-1][1] - nodes[i][1] > 1 {
                    nodes[i][1] += 1;
                    if nodes[i-1][0] - nodes[i][0] > 0 {
                        nodes[i][0] += 1;
                    }
                    if nodes[i][0] - nodes[i-1][0] > 0 {
                        nodes[i][0] -= 1;
                    }
                }

                // Chase up
                if nodes[i][1] - nodes[i-1][1] > 1 {
                    nodes[i][1] -= 1;
                    // Check diagonals
                    if nodes[i-1][0] - nodes[i][0] > 0 {
                        nodes[i][0] += 1;
                    }
                    if nodes[i][0] - nodes[i-1][0] > 0 {
                        nodes[i][0] -= 1;
                    }
                }
            }

            visited.insert(nodes[nodes.len()-1]);
        }
    }

    println!("{visited:?}");
    println!("part two: {}", visited.len());
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
