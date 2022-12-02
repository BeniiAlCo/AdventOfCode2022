pub fn run() {
    let input = include_str!("input/day1.txt");
    let processed_input = sum_groups(input);
    println!("{}", puzzle_1(&processed_input));
    println!("{}", puzzle_2(&processed_input));
}

// Each elf is carrying x calories worth of food
// x is a number, or the sum of multiple numbers on consequtive lines, seperated by an empty line

fn sum_groups(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|element| element.parse::<u32>().expect("Unexpected value -- either not a number, a floating point number, or a huge (>u32) number."))
                .sum()
        })
        .collect()
}

// Puzzle 1:
// How many calories are being carried by the elf carrying the most calories ?
fn puzzle_1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

//Puzzle 2:
// How many calories are carried by the top three calorie-carrying elves ?
fn puzzle_2(input: &[u32]) -> u32 {
    let mut input = input.to_owned();
    input.sort();
    input.split_off(input.len() - 3).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn parse_input() {
        assert_eq!(
            sum_groups(TEST_INPUT),
            vec![6000, 4000, 11000, 24000, 10000]
        );
    }

    #[test]
    fn puzzle_1_on_test_input() {
        assert_eq!(puzzle_1(&sum_groups(TEST_INPUT)), 24000);
    }

    #[test]
    fn puzzle_2_on_test_input() {
        assert_eq!(puzzle_2(&sum_groups(TEST_INPUT)), 45000);
    }
}
