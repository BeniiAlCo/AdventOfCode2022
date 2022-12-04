use std::ops::RangeInclusive;

pub fn run() {
    let input = include_str!("input/day_4.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// Elves have been assigned to clear space in the camp.
// Each section of the camp has a unique ID number.
// Some sections overlap.
// Each assigned elf clears an assigned range of section IDs.
//
// Elves want to identify overlapping sections, as to reduce duplicated clearing efforts.
// To do this they pair up, and make a list of assigned area ranges.
// Input is a list of pairs of comma-seperated inclusive ranges

fn parse_input(input: &str) -> Vec<Vec<RangeInclusive<u64>>> {
    input
        .lines()
        .map(|pair| {
            pair.split(',')
                .map(|area| {
                    let bounds = area
                        .split('-')
                        .map(|bound| bound.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    RangeInclusive::new(bounds[0], bounds[1])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

// In how many assignment pairs does one range fully contain the other?
fn puzzle_1(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|pair| {
            (pair[0].contains(pair[1].start()) && pair[0].contains(pair[1].end()))
                || (pair[1].contains(pair[0].start()) && pair[1].contains(pair[0].end()))
        })
        .count() as u64
}

// How many assignment pairs overlap at all?
fn puzzle_2(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|pair| {
            pair[0].contains(pair[1].start())
                || pair[0].contains(pair[1].end())
                || pair[1].contains(pair[0].start())
                || pair[1].contains(pair[0].end())
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn day_4_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                vec![2..=4, 6..=8],
                vec![2..=3, 4..=5],
                vec![5..=7, 7..=9],
                vec![2..=8, 3..=7],
                vec![6..=6, 4..=6],
                vec![2..=6, 4..=8]
            ]
        );
    }

    #[test]
    fn day_4_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 2);
    }

    #[test]
    fn day_4_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 4);
    }
}
