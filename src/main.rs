use std::fs;
use std::env;
use rps;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_contents = fs::read_to_string(&args[1]).unwrap();

    let mut final_score: u32 = 0;
    let mut final_score_corrected: u32 = 0;
    for round in file_contents.lines() {
        let encode: Vec<char> = round.chars().collect();
        final_score = final_score + rps::round_result(&encode[0..=2]);
        final_score_corrected = final_score_corrected + rps::round_result_corrected(&encode[0..=2]);
    }

    println!("Final score from tourney: {}", final_score);
    println!("Final score from tourney with correct encoding: {}", final_score_corrected);
}

