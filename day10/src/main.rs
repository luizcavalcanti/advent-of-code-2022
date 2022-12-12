// use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let file_content = read_file("input.txt");
    let lines = file_content.split("\n");

    let mut total = 0;
    let mut x = 1;
    let mut cycle = 0;
    let mut raster = String::new();

    for line in lines {
        if line == "" {
            continue;
        }

        if line == "noop" {
            cycle += 1;

            if cycle == 20 || (cycle - 20) % 40 == 0 {
                total += x * cycle;
            }

            let pos = cycle % 40;
            if ((pos - 1 - x) as i32).abs() < 2 {
                raster.push_str("#");
            } else {
                raster.push_str(".");
            }
            if pos == 0 {
                raster.push_str("\n");
            }
        } else {
            let command: Vec<&str> = line.split(" ").collect();
            let ammount: i32 = FromStr::from_str(command[1]).unwrap();
            for _ in 0..2 {
                cycle += 1;

                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    total += x * cycle;
                }

                let pos = cycle % 40;
                if ((pos - 1 - x) as i32).abs() < 2 {
                    raster.push_str("#");
                } else {
                    raster.push_str(".");
                }
                if pos == 0 {
                    raster.push_str("\n");
                }
            }

            x += ammount;
        }
    }
    println!("part one: {total}");
    println!("part two: \n{raster}");
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
