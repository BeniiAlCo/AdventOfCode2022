pub fn run() {
    let input = include_str!("input/day_5.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// Supplies need to be unloaded from the ship.
// Supplies are stored in stacks of marked crates, behind other, non-marked crates.
// To get to them, the crates will need to be rearranged.
//
// The ship can move crates between stacks.
// When a crate is moved, crates are moved around until the selected crate is at the top of a
// stack.
//
// To figure out which crate will end up where, the elves have the starting position, and the
// rearrangement procedure.
// When (a) crate(s) is/are moved from one stack to another, it is the top crate(s) that is/are moved, and it/they is/are
// placed onto the top of the stack that it/they is/are moved to.

#[derive(Debug, PartialEq)]
struct Rearrangement {
    number: u64,
    origin: u64,
    destination: u64,
}

#[derive(Debug, PartialEq)]
struct Procedure {
    stack_state: Vec<Vec<char>>,
    rearrangements: Vec<Rearrangement>,
}

fn parse_input(input: &str) -> Procedure {
    let number_of_stacks = input
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|number| number.parse::<usize>().ok())
        })
        .unwrap();

    let mut stack_state = Vec::with_capacity(number_of_stacks);
    (0..number_of_stacks).for_each(|idx| {
        stack_state.push(
            input
                .lines()
                .take_while(|line| line.contains('['))
                .filter_map(|line| {
                    line.chars()
                        .nth((idx * 4) + 1)
                        .filter(|char| char.is_alphabetic())
                })
                .collect::<Vec<_>>(),
        );
        stack_state[idx].reverse();
    });

    let rearrangements = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|rearrangements| {
            let mut r = rearrangements
                .split_whitespace()
                .filter_map(|number| number.parse::<u64>().ok());
            Rearrangement {
                number: r.next().unwrap(),
                origin: r.next().unwrap(),
                destination: r.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    Procedure {
        stack_state,
        rearrangements,
    }
}

//
fn puzzle_1(input: &str) -> String {
    let mut input = parse_input(input);
    input.rearrangements.iter().for_each(|rearrangement| {
        let len = input.stack_state[rearrangement.origin as usize - 1].len();
        let moving_crates = &mut input.stack_state[rearrangement.origin as usize - 1]
            .split_off(len - rearrangement.number as usize);
        moving_crates.reverse();
        input.stack_state[rearrangement.destination as usize - 1].append(moving_crates)
    });
    input
        .stack_state
        .into_iter()
        .map(|mut stack| stack.pop().unwrap_or(' '))
        .collect::<String>()
}

//
fn puzzle_2(input: &str) -> String {
    let mut input = parse_input(input);
    input.rearrangements.iter().for_each(|rearrangement| {
        let len = input.stack_state[rearrangement.origin as usize - 1].len();
        let moving_crates = &mut input.stack_state[rearrangement.origin as usize - 1]
            .split_off(len - rearrangement.number as usize);
        input.stack_state[rearrangement.destination as usize - 1].append(moving_crates)
    });
    input
        .stack_state
        .into_iter()
        .map(|mut stack| stack.pop().unwrap_or(' '))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

    #[test]
    fn day_5_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            Procedure {
                rearrangements: vec![
                    Rearrangement {
                        number: 1,
                        origin: 2,
                        destination: 1
                    },
                    Rearrangement {
                        number: 3,
                        origin: 1,
                        destination: 3
                    },
                    Rearrangement {
                        number: 2,
                        origin: 2,
                        destination: 1
                    },
                    Rearrangement {
                        number: 1,
                        origin: 1,
                        destination: 2
                    }
                ],
                stack_state: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            }
        );
    }

    #[test]
    fn day_5_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn day_5_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), "MCD");
    }
}
