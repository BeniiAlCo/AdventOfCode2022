pub fn run() {
	//let input = include_str!("input/day_9");
	println!("{}", puzzle_1(input));
	println!("{}", puzzle_2(input));
}

// 

fn parse_input(input: &str) -> Vec<u64> {
	unimplemented!()
}

// 
fn puzzle_1(input: &str) -> u64 {
	unimplemented!()
}

// 
fn puzzle_2(input: &str) -> u64 {
	unimplemented!()
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = "";

	#[test]
	fn day_9_parse_input() {
		assert_eq!(parse_input(TEST_INPUT), vec![0]);
	}

	#[test]
	fn day_9_puzzle_1() {
		assert_eq!(puzzle_1(TEST_INPUT), 0);
	}

	#[test]
	fn day_9_puzzle_2() {
		assert_eq!(puzzle_2(TEST_INPUT), 0);
	}
}
