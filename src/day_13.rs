use std::cmp::Ordering;

pub fn run() {
    let input = include_str!("input/day_13.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

//

fn parse_input(input: &str) -> Vec<Vec<(Vec<i64>, usize)>> {
    input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| {
                    (
                        line.replace("[]", "-1")
                            .replace([',', '[', ']'], " ")
                            .split_whitespace()
                            .map(|digit| digit.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>(),
                        line.chars().filter(|&c| c == '[' || c == ']').count(),
                    )
                })
                .collect::<Vec<(Vec<i64>, usize)>>()
        })
        .collect::<Vec<Vec<(Vec<i64>, usize)>>>()
}

//
fn puzzle_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .enumerate()
        .map(|(idx, pair)| {
            let (left, left_length) = &pair[0];
            let (right, right_length) = &pair[1];

            match left.iter().cmp(right.iter()) {
                Ordering::Equal => {
                    if right_length < left_length {
                        0
                    } else {
                        idx + 1
                    }
                }
                Ordering::Less => idx + 1,
                Ordering::Greater => 0,
            }
        })
        .sum()
}

//
fn puzzle_2(input: &str) -> usize {
    let mut inputs = parse_input(input);
    inputs.push(vec![(vec![2], 5), (vec![6], 5)]);
    let mut inputs = inputs
        .into_iter()
        .flatten()
        .map(|(line, _size)| line)
        .collect::<Vec<Vec<i64>>>();
    inputs.sort();

    let a = inputs
        .iter()
        .position(|line| line.len() == 1 && line[0] == 2)
        .unwrap()
        + 1;
    let b = &inputs
        .iter()
        .position(|line| line.len() == 1 && line[0] == 6)
        .unwrap()
        + 1;
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn day_13_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                vec![(vec![1, 1, 3, 1, 1], 2), (vec![1, 1, 5, 1, 1], 2)],
                vec![(vec![1, 2, 3, 4], 6), (vec![1, 4], 4)],
                vec![(vec![9], 2), (vec![8, 7, 6], 4)],
                vec![(vec![4, 4, 4, 4], 4), (vec![4, 4, 4, 4, 4], 4)],
                vec![(vec![7, 7, 7, 7], 2), (vec![7, 7, 7], 2)],
                vec![(vec![-1], 2), (vec![3], 2)],
                vec![(vec![-1], 6), (vec![-1], 4)],
                vec![
                    (vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 10),
                    (vec![1, 2, 3, 4, 5, 6, 0, 8, 9], 10)
                ]
            ]
        );
    }

    #[test]
    fn day_13_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 13);
        //panic!();
    }

    #[test]
    fn day_13_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 140);
    }
}
