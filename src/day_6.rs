pub fn run() {
    let input = include_str!("input/day_6.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// The elves and I leave for the star fruit grove.
// As we travel, an elf gives me a malfunctioning communications device.
//
// To communicate, the device needs to lock on to a seemingly-random stream of characters.
// To fix the malfunction, we add a subroutine that detects the 'start_of_packet' marker.
// The marker is a sequence of 4 unique characters.
//
// Identify the first 4 contiguous unique characters.

fn parse_input(input: &str, window_size: usize) -> &str {
    input
        .as_bytes()
        .windows(window_size)
        .find(|seq| !(1..seq.len()).any(|i| seq[i..].contains(&seq[i - 1])))
        .map(std::str::from_utf8)
        .unwrap()
        .unwrap()
}

// How many characters need to be processed before the first start_of_packet marker?
fn puzzle_1(input: &str) -> usize {
    let marker_length = 4;
    input.find(parse_input(input, marker_length)).unwrap() + marker_length
}

// A 'start_of_message' marker is a sequence of 14 unique characters.
// How many characters need to be processed before the first start_of_message marker is detected?
fn puzzle_2(input: &str) -> usize {
    let marker_length = 14;
    input.find(parse_input(input, marker_length)).unwrap() + marker_length
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";

    #[test]
    fn day_6_parse_input() {
        assert_eq!(parse_input(TEST_INPUT_1, 4), "jpqm");
        assert_eq!(parse_input(TEST_INPUT_2, 4), "vwbj");
        assert_eq!(parse_input(TEST_INPUT_3, 4), "pdvj");
    }

    #[test]
    fn day_6_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT_1), 7);
        assert_eq!(puzzle_1(TEST_INPUT_2), 5);
        assert_eq!(puzzle_1(TEST_INPUT_3), 6);
    }

    #[test]
    fn day_6_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT_1), 19);
        assert_eq!(puzzle_2(TEST_INPUT_2), 23);
        assert_eq!(puzzle_2(TEST_INPUT_3), 23);
    }
}
