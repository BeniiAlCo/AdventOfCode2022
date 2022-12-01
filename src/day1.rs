pub fn run() {
    let input = include_str!("input/day1.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// Each elf is carrying x calories worth of food
// x is a number, or the sum of multiple numbers on consequtive lines, seperated by an empty line

// Puzzle 1:
// How many calories are being carried by the elf carrying the most calories ?
fn puzzle_1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|item| item
                .parse::<u32>()
                .expect("Unexpected non-numeric input (or a numeric input that is too big, or a float!)"))
            .sum())
        .max()
        .unwrap()
}

//Puzzle 2:
// How many calories are carried by the top three calorie-carrying elves ?
fn puzzle_2(input: &str) -> u32 {
    let mut input = input
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|item| item
                .parse::<u32>()
                .expect("Unexpected non-numeric input (or a numeric input that is too big, or a float!)"))
            .sum())
        .collect::<Vec<u32>>();
    input.sort();
    input.split_off(input.len() - 3).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn puzzle_1_on_test_input() {
        assert_eq!(puzzle_1(TEST_INPUT), 24000);
    }

    #[test]
    fn puzzle_2_on_test_input() {
        assert_eq!(puzzle_2(TEST_INPUT), 45000);
    }
}
