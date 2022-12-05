use std::fs;
use std::str::FromStr;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let stacks = move_stuff(false);

    let mut output = String::new();

    for stk in stacks {
        output.push(*stk.last().unwrap());
    }

    println!("part one: {}", output);
}

fn part_two() {
    let stacks = move_stuff(true);

    let mut output = String::new();

    for stk in stacks {
        output.push(*stk.last().unwrap());
    }

    println!("part one: {}", output);
}

fn move_stuff(all_at_once: bool) -> Vec<Vec<char>> {
    let file_content = read_file("input.txt");

    let mut stacks: Vec<Vec<char>> = init_stacks();

    let lines = file_content.split("\n");

    for line in lines {
        if !line.starts_with("move") {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        let ammount: i32 = FromStr::from_str(parts[1]).unwrap();
        let origin: usize = FromStr::from_str(parts[3]).unwrap();
        let destination: usize = FromStr::from_str(parts[5]).unwrap();

        if all_at_once {
            let mut transfer_stack: Vec<char> = Vec::new();

            for _ in 0..ammount {
                let obj = stacks[origin - 1].pop().unwrap();
                transfer_stack.push(obj);
            }

            while !transfer_stack.is_empty() {
                let obj = transfer_stack.pop().unwrap();
                stacks[destination - 1].push(obj);
            }
        } else {
            for _ in 0..ammount {
                let obj = stacks[origin - 1].pop().unwrap();
                stacks[destination - 1].push(obj);
            }
        }
    }

    return stacks;
}

//                         [Z] [W] [Z]
//         [D] [M]         [L] [P] [G]
//     [S] [N] [R]         [S] [F] [N]
//     [N] [J] [W]     [J] [F] [D] [F]
// [N] [H] [G] [J]     [H] [Q] [H] [P]
// [V] [J] [T] [F] [H] [Z] [R] [L] [M]
// [C] [M] [C] [D] [F] [T] [P] [S] [S]
// [S] [Z] [M] [T] [P] [C] [D] [C] [D]
//  1   2   3   4   5   6   7   8   9
fn init_stacks() -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    stacks.push(Vec::new());
    stacks[0].push('S');
    stacks[0].push('C');
    stacks[0].push('V');
    stacks[0].push('N');

    stacks.push(Vec::new());
    stacks[1].push('Z');
    stacks[1].push('M');
    stacks[1].push('J');
    stacks[1].push('H');
    stacks[1].push('N');
    stacks[1].push('S');

    stacks.push(Vec::new());
    stacks[2].push('M');
    stacks[2].push('C');
    stacks[2].push('T');
    stacks[2].push('G');
    stacks[2].push('J');
    stacks[2].push('N');
    stacks[2].push('D');

    stacks.push(Vec::new());
    stacks[3].push('T');
    stacks[3].push('D');
    stacks[3].push('F');
    stacks[3].push('J');
    stacks[3].push('W');
    stacks[3].push('R');
    stacks[3].push('M');

    stacks.push(Vec::new());
    stacks[4].push('P');
    stacks[4].push('F');
    stacks[4].push('H');

    stacks.push(Vec::new());
    stacks[5].push('C');
    stacks[5].push('T');
    stacks[5].push('Z');
    stacks[5].push('H');
    stacks[5].push('J');

    stacks.push(Vec::new());
    stacks[6].push('D');
    stacks[6].push('P');
    stacks[6].push('R');
    stacks[6].push('Q');
    stacks[6].push('F');
    stacks[6].push('S');
    stacks[6].push('L');
    stacks[6].push('Z');

    stacks.push(Vec::new());
    stacks[7].push('C');
    stacks[7].push('S');
    stacks[7].push('L');
    stacks[7].push('H');
    stacks[7].push('D');
    stacks[7].push('F');
    stacks[7].push('P');
    stacks[7].push('W');

    stacks.push(Vec::new());
    stacks[8].push('D');
    stacks[8].push('S');
    stacks[8].push('M');
    stacks[8].push('P');
    stacks[8].push('F');
    stacks[8].push('N');
    stacks[8].push('G');
    stacks[8].push('Z');

    return stacks;
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
