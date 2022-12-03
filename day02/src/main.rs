use std::fs;

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const PLAYER_ROCK: &str = "X";
const PLAYER_PAPER: &str = "Y";
const PLAYER_SCISSORS: &str = "Z";

const OUTCOME_LOSE: &str = "X";
const OUTCOME_DRAW: &str = "Y";
const OUTCOME_WIN: &str = "Z";

fn main() {
    let file_content = read_file("input.txt");
    let lines = file_content.split("\n");

    let mut total: u32 = 0;
    let mut outcome_total: u32 = 0;
    for game in lines {
        if game == "" {
            continue;
        }
        let mut inputs = game.split(" ");
        let opponent = inputs.next().unwrap();
        let player = inputs.next().unwrap();
        let outcome = define_outcome(opponent, player);

        total += calculate_score(opponent, player);
        outcome_total += calculate_score(opponent, outcome);
    }

    println!("Part one: {}", total);
    println!("Part two: {}", outcome_total);
}

fn calculate_score(opponent: &str, player: &str) -> u32 {
    let mut total: u32 = 0;

    total += match opponent {
        OPPONENT_ROCK => match player {
            PLAYER_ROCK => SCORE_DRAW,
            PLAYER_PAPER => SCORE_WIN,
            PLAYER_SCISSORS | _ => 0,
        },
        OPPONENT_PAPER => match player {
            PLAYER_PAPER => SCORE_DRAW,
            PLAYER_SCISSORS => SCORE_WIN,
            PLAYER_ROCK | _ => 0,
        },
        OPPONENT_SCISSORS | _ => match player {
            PLAYER_SCISSORS => SCORE_DRAW,
            PLAYER_ROCK => SCORE_WIN,
            PLAYER_PAPER | _ => 0,
        },
    };

    total += match player {
        PLAYER_ROCK => SCORE_ROCK,
        PLAYER_PAPER => SCORE_PAPER,
        PLAYER_SCISSORS | _ => SCORE_SCISSORS,
    };

    return total;
}

fn define_outcome<'a>(opponent: &str, desired_outcome: &'a str) -> &'a str {
    return match opponent {
        OPPONENT_ROCK => match desired_outcome {
            OUTCOME_LOSE => PLAYER_SCISSORS,
            OUTCOME_DRAW => PLAYER_ROCK,
            OUTCOME_WIN | _ => PLAYER_PAPER,
        },
        OPPONENT_PAPER => match desired_outcome {
            OUTCOME_LOSE => PLAYER_ROCK,
            OUTCOME_DRAW => PLAYER_PAPER,
            OUTCOME_WIN | _ => PLAYER_SCISSORS,
        },
        OPPONENT_SCISSORS | _ => match desired_outcome {
            OUTCOME_LOSE => PLAYER_PAPER,
            OUTCOME_DRAW => PLAYER_SCISSORS,
            OUTCOME_WIN | _ => PLAYER_ROCK,
        },
    };
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
