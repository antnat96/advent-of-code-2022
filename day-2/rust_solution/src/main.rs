// Started here: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;

const OPPONENT_ROCK_LETTER: &str = "A";
const OPPONENT_PAPER_LETTER: &str = "B";
const OPPONENT_SCISSORS_LETTER: &str = "C";

const YOUR_ROCK_LETTER: &str = "X";
const YOUR_PAPER_LETTER: &str = "Y";
const YOUR_SCISSORS_LETTER: &str = "Z";

const LOST_POINTS: i32 = 0;
const DRAW_POINTS: i32 = 3;
const WIN_POINTS: i32 = 6;

fn determine_play_value(you_played: &str) -> i32 {
    if you_played == YOUR_ROCK_LETTER {
        return 1;
    } else if you_played == YOUR_PAPER_LETTER {
        return 2;
    } else if you_played == YOUR_SCISSORS_LETTER {
        return 3;
    }
    return 0;
}

fn determine_outcome_value(you_played: &str, opponent_played: &str) -> i32 {
    if opponent_played == OPPONENT_ROCK_LETTER {
        if you_played == YOUR_ROCK_LETTER {
            return DRAW_POINTS;
        }
        if you_played == YOUR_PAPER_LETTER {
            return WIN_POINTS;
        }
        if you_played == YOUR_SCISSORS_LETTER {
            return LOST_POINTS;
        }
    }

    if opponent_played == OPPONENT_PAPER_LETTER {
        if you_played == YOUR_PAPER_LETTER {
            return DRAW_POINTS;
        }
        if you_played == YOUR_SCISSORS_LETTER {
            return WIN_POINTS;
        }
        if you_played == YOUR_ROCK_LETTER {
            return LOST_POINTS;
        }
    }

    if opponent_played == OPPONENT_SCISSORS_LETTER {
        if you_played == YOUR_SCISSORS_LETTER {
            return DRAW_POINTS;
        }
        if you_played == YOUR_ROCK_LETTER {
            return WIN_POINTS;
        }
        if you_played == YOUR_PAPER_LETTER {
            return LOST_POINTS;
        }
    }
    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut overall_points: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let arr: Vec<&str> = ip.split(" ").collect();
                let opponent_played: &str = arr[0];
                let you_played: &str = arr[1];
                let play_value: i32 = determine_play_value(you_played);
                let outcome_value: i32 = determine_outcome_value(you_played, opponent_played);
                overall_points += play_value + outcome_value;
            }
        }
        println!("{}", overall_points);
    }
}
