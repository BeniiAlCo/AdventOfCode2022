pub fn run() {
    let input = include_str!("input/day2.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// A Rock, Paper, Scissors tournament.
// We have an encrypted strategy guide (input):
// Made up of two entries per line -- '<PREDICTION> <OTHER>'
// PREDICTION is what the opponent will play,
// A = Rock
// B = Paper
// C = Scissors
//
// Winner has the highest total score -- sum of all round scores
// Round score = shape (Rock = 1; Paper = 2; Scissors = 3) + outcome (loss = 0; draw = 3; win = 6)

fn parse_input(input: &str, puzzle_map: fn((&str, &str)) -> u32) -> Vec<u32> {
    input
        .lines()
        .map(|round| round.split_once(' ').map(puzzle_map).unwrap())
        .collect()
}

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

// Puzzle 1:
// OTHER is what we should play in response,
// X = Rock
// Y = Paper
// Z = Scissors

fn puzzle_1_map(input: (&str, &str)) -> u32 {
    match input {
        ("A", "X") => ROCK + DRAW,
        ("A", "Y") => PAPER + WIN,
        ("A", "Z") => SCISSORS + LOSE,
        ("B", "X") => ROCK + LOSE,
        ("B", "Y") => PAPER + DRAW,
        ("B", "Z") => SCISSORS + WIN,
        ("C", "X") => ROCK + WIN,
        ("C", "Y") => PAPER + LOSE,
        ("C", "Z") => SCISSORS + DRAW,
        _ => unreachable!(),
    }
}

// Calculate the score if we were to follow the strategy guide
fn puzzle_1(input: &str) -> u32 {
    parse_input(input, puzzle_1_map).iter().sum()
}

// Puzzle 2:
// OTHER is how the round should end,
// X = Lose
// Y = Draw
// Z = Win

fn puzzle_2_map(input: (&str, &str)) -> u32 {
    match input {
        ("A", "X") => SCISSORS + LOSE,
        ("A", "Y") => ROCK + DRAW,
        ("A", "Z") => PAPER + WIN,
        ("B", "X") => ROCK + LOSE,
        ("B", "Y") => PAPER + DRAW,
        ("B", "Z") => SCISSORS + WIN,
        ("C", "X") => PAPER + LOSE,
        ("C", "Y") => SCISSORS + DRAW,
        ("C", "Z") => ROCK + WIN,
        _ => unreachable!(),
    }
}

// Calculate the score if we follow this guide and chose the correct corresponding shape
fn puzzle_2(input: &str) -> u32 {
    parse_input(input, puzzle_2_map).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn day_2_puzzle_1_parse_input() {
        assert_eq!(parse_input(TEST_INPUT, puzzle_1_map), vec![8, 1, 6]);
    }

    #[test]
    fn day_2_puzzle_2_parse_input() {
        assert_eq!(parse_input(TEST_INPUT, puzzle_2_map), vec![4, 1, 7])
    }

    #[test]
    fn day_2_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 15);
    }

    #[test]
    fn day_2_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 12);
    }
}
