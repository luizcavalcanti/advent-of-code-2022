use std::fs;

fn main() {
    let forest = load_forest();
    count_visible_trees(&forest);
    get_best_scenic_score(&forest);
}

fn get_best_scenic_score(forest: &Vec<Vec<u8>>) {
    let mut best_score = 0;
    let height = forest.len();
    let width = forest[0].len();

    for i in 0..height {
        for j in 0..width {
            let tree_size = forest[i][j];

            let mut score_up = 0;
            for x in (0..i).rev() {
                score_up += 1;
                if forest[x][j] >= tree_size {
                    break;
                }
            }

            let mut score_down = 0;
            for x in i + 1..height {
                score_down += 1;
                if forest[x][j] >= tree_size {
                    break;
                }
            }

            let mut score_left = 0;
            for x in (0..j).rev() {
                score_left += 1;
                if forest[i][x] >= tree_size {
                    break;
                }
            }

            let mut score_right = 0;
            for x in j + 1..width {
                score_right += 1;
                if forest[i][x] >= tree_size {
                    break;
                }
            }

            let score = score_up * score_down * score_left * score_right;
            if score > best_score {
                best_score = score;
            }
        }
    }

    println!("part two: {best_score}");
}

fn count_visible_trees(forest: &Vec<Vec<u8>>) {
    let height = forest.len();
    let width = forest[0].len();

    let mut visible_count = 2 * width + 2 * height - 4;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let tree_size = forest[i][j];

            let mut visible_upwards = true;
            for v in 0..i {
                if tree_size <= forest[v][j] {
                    visible_upwards = false;
                }
            }
            let mut visible_downwards = true;
            for v in i + 1..height {
                if tree_size <= forest[v][j] {
                    visible_downwards = false;
                }
            }

            let mut visible_leftwards = true;
            for h in 0..j {
                if tree_size <= forest[i][h] {
                    visible_leftwards = false;
                }
            }
            let mut visible_rightwards = true;
            for h in j + 1..width {
                if tree_size <= forest[i][h] {
                    visible_rightwards = false;
                }
            }

            if visible_upwards || visible_downwards || visible_leftwards || visible_rightwards {
                visible_count += 1;
            }
        }
    }

    println!("part one: {visible_count}");
}

fn load_forest() -> Vec<Vec<u8>> {
    let file_content = read_file("input.txt");
    let mut field: Vec<Vec<u8>> = Vec::new();

    let lines = file_content.split("\n");
    for line in lines {
        if line == "" {
            continue;
        }

        let mut line_vec: Vec<u8> = Vec::new();
        for c in line.chars() {
            line_vec.push(c.to_digit(10).unwrap() as u8);
        }
        field.push(line_vec);
    }
    return field;
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
