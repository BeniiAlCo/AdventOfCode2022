pub fn run() {
    let input = include_str!("input/day_1.txt");
    //let processed_input = &mut sum_groups(input);
    //processed_input.sort();
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// Each elf is carrying x calories worth of food
// x is a single number on a single line, or the sum of multiple numbers on consequtive lines.
// Each x is seperated by an empty line.

fn sum_groups(input: &str) -> Vec<u64> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|elf| {
            elf
                .lines()
                .map(|food| food.parse::<u64>().expect("Unexpected value -- either not a number, a floating point number, or a huge (>u64) number."))
                .sum()
        })
        .collect()
}

// Puzzle 1:
// How many calories are being carried by the elf carrying the most calories ?
fn puzzle_1(input: &str) -> u64 {
    sum_groups(input).iter().max().copied().unwrap()
}

//Puzzle 2:
// How many calories are carried by the top three calorie-carrying elves ?
fn puzzle_2(input: &str) -> u64 {
    let input = &mut sum_groups(input);
    input.sort();
    input.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn day_1_parse_input() {
        assert_eq!(
            sum_groups(TEST_INPUT),
            vec![6000, 4000, 11000, 24000, 10000]
        );
    }

    #[test]
    fn day_1_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 24000);
    }

    #[test]
    fn day_1_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 45000);
    }
}
