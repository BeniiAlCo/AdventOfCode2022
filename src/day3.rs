pub fn run() {
    let input = include_str!("input/day3.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// An elf loads rucksacks with supplies.
// Those supplies need to be rearranged.
//
// A rucksack has two compartments.
// Each item-kind goes into only one of the compartments.
// As given, one item-kind per rucksack is incorrect.
//
// Input is a list of all items in each rucksack:
// One letter per item (upper and lower case are distinct); one line per rucksack.
// Compartment 1 = first half of the letters on a line,
// Compartment 2 = second half of the letters on a line.

fn ascii_to_number(input: char) -> u64 {
    if input.is_ascii_uppercase() {
        input as u64 - 38
    } else if input.is_ascii_lowercase() {
        input as u64 - 96
    } else {
        panic!("aa")
    }
}

// Puzzle 1:
// a..z have priority 1..26
// A..Z have priority 27..52
//
// What is the sum of the priorities of the item types that appear in both compartments?
fn puzzle_1(input: &str) -> u64 {
    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(compartment_1, compartment_2)| {
            compartment_1
                .chars()
                .find(|&char_1| compartment_2.contains(char_1))
                .map(ascii_to_number)
                .unwrap()
        })
        .sum()
}

// Puzzle 2:
// Elves are divided into groups of three.
// Within each group of three, a group-identifying badge is the only item common to all
// The letter identifying each badge can be different between groups.
//
// What is the sum of the priorities of each group's badge item-kind
fn puzzle_2(input: &str) -> u64 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let member_1 = group[0];
            let member_2 = group[1];
            let member_3 = group[2];

            member_1
                .chars()
                .find(|&char_1| member_2.contains(char_1) && member_3.contains(char_1))
                .map(ascii_to_number)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    fn day_3_ascii_to_numbers() {
        assert_eq!(ascii_to_number('a'), 1);
        assert_eq!(ascii_to_number('A'), 27);
        assert_eq!(ascii_to_number('z'), 26);
        assert_eq!(ascii_to_number('Z'), 52);
    }

    fn day_3_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 157);
    }

    fn day_3_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 70);
    }
}
