#[derive(Debug, PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw
}

fn determine_result(choices: (&Choice, &Choice)) -> RoundResult {
    match choices {
        (Choice::Scissors, Choice::Paper) 
            | (Choice::Paper, Choice::Rock) 
            | (Choice::Rock, Choice::Scissors) => RoundResult::Win,
        (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock)
            | (Choice::Rock, Choice::Paper) => RoundResult::Lose,
        _ => RoundResult::Draw,
    }
}

fn score(choice: &Choice, result: &RoundResult) -> u32 {
    let result_score = match result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Lose => 0,
    };

    let choice_score = match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    result_score + choice_score
}

fn convert_opp(choice: char) -> Choice {
    match choice {
        'A' | 'a' => Choice::Rock,
        'B' | 'b' => Choice::Paper,
        'C' | 'c' => Choice::Scissors,
        _ => panic!("Illegal character found"),
    }
}

fn convert_prot(choice: char) -> Choice {
    match choice {
        'X' | 'x' => Choice::Rock,
        'Y' | 'y' => Choice::Paper,
        'Z' | 'z' => Choice::Scissors,
        _ => panic!("Illegal character found"),
    }
}

fn convert_result(char: char) -> RoundResult {
    match char {
        'X' | 'x' => RoundResult::Lose,
        'Y' | 'y' => RoundResult::Draw,
        'Z' | 'z' => RoundResult::Win,
        _ => panic!("Illegal character found"),
    }
}

fn get_choice_from_result(result: &RoundResult, opp: &Choice,) -> Choice {
    match result {
        RoundResult::Draw => *opp,
        RoundResult::Win => match opp {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
        RoundResult::Lose => match opp {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
    }
}

pub fn round_result(chars: &[char]) -> u32 {
    let choices = (&convert_prot(chars[2]), &convert_opp(chars[0]));
    let result = determine_result(choices);
    score(choices.0, &result)
}

pub fn round_result_corrected(chars: &[char]) -> u32 {
    let desired_result = convert_result(chars[2]);
    let opponent_choice = convert_opp(chars[0]);
    let prot_choice = get_choice_from_result(&desired_result, &opponent_choice);
    
    score(&prot_choice, &desired_result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_cons() {
        let result = determine_result((&Choice::Scissors, &Choice::Paper));
        assert_eq!(result, RoundResult::Win);
        let result = determine_result((&Choice::Paper, &Choice::Rock));
        assert_eq!(result, RoundResult::Win);
        let result = determine_result((&Choice::Rock, &Choice::Scissors));
        assert_eq!(result, RoundResult::Win);
    }

    #[test]
    fn test_loss_cons() {
        let result = determine_result((&Choice::Paper, &Choice::Scissors));
        assert_eq!(result, RoundResult::Lose);
        let result = determine_result((&Choice::Rock, &Choice::Paper));
        assert_eq!(result, RoundResult::Lose);
        let result = determine_result((&Choice::Scissors, &Choice::Rock));
        assert_eq!(result, RoundResult::Lose);
    }

    #[test]
    fn test_draw_cons() {
        let result = determine_result((&Choice::Paper, &Choice::Paper));
        assert_eq!(result, RoundResult::Draw);
        let result = determine_result((&Choice::Rock, &Choice::Rock));
        assert_eq!(result, RoundResult::Draw);
        let result = determine_result((&Choice::Scissors, &Choice::Scissors));
        assert_eq!(result, RoundResult::Draw);
    }

    #[test]
    fn test_scoring_loss() {
        let result = score(&Choice::Rock, &RoundResult::Lose);
        assert_eq!(result, 1);
        let result = score(&Choice::Paper, &RoundResult::Lose);
        assert_eq!(result, 2);
        let result = score(&Choice::Scissors, &RoundResult::Lose);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_scoring_draw() {
        let result = score(&Choice::Rock, &RoundResult::Draw);
        assert_eq!(result, 4);
        let result = score(&Choice::Paper, &RoundResult::Draw);
        assert_eq!(result, 5);
        let result = score(&Choice::Scissors, &RoundResult::Draw);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_scoring_win() {
        let result = score(&Choice::Rock, &RoundResult::Win);
        assert_eq!(result, 7);
        let result = score(&Choice::Paper, &RoundResult::Win);
        assert_eq!(result, 8);
        let result = score(&Choice::Scissors, &RoundResult::Win);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_full_stack() {
        let result = round_result(&['A', ' ', 'Y']);
        assert_eq!(result, 8);
        let result = round_result(&['B', ' ', 'X']);
        assert_eq!(result, 1);
        let result = round_result(&['C', ' ', 'Z']);
        assert_eq!(result, 6);
    }
}

