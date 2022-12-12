use std::collections::{HashSet, VecDeque};

pub fn run() {
    let input = include_str!("input/day_12.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// We don't have a good enough signal to contact the elves.
// We ask for a height map of the area.
// The map is broken into a grid.
// Each square of the grid shows an elevation a..=z, where a is the lowest+.
// 'S' is the current location.
// 'E' is the position with the best signal.
// We want to move from S to E in the fewest steps possible.
// We can only move up, down, left, or right.
// We can only move to squares of at most one level of elevation higher than our current square.

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    point: Point,
    letter: char,
}

impl Position {
    fn start(grid: &[Vec<char>]) -> Self {
        Self::letter_position(grid, 'S')[0]
    }
    fn end(grid: &[Vec<char>]) -> Self {
        Self::letter_position(grid, 'E')[0]
    }
    fn letter_position(grid: &[Vec<char>], target_letter: char) -> Vec<Self> {
        grid.iter()
            .enumerate()
            .filter_map(|(y, line)| {
                let row = line
                    .iter()
                    .enumerate()
                    .find(|(_x, &letter)| letter == target_letter);
                row.map(|(x, &letter)| Position {
                    point: Point { x, y },
                    letter,
                })
            })
            .collect::<Vec<Position>>()
    }
    fn neighbours(&self, grid: &[Vec<char>]) -> Vec<Self> {
        fn is_valid(current: char, other: char) -> bool {
            let current = if current == 'S' { 'a' } else { current };
            let other = if other == 'E' { 'z' } else { other };

            other <= (current as u8 + 1) as char
        }

        let mut neighbours = Vec::new();

        if self.point.x < grid[self.point.y].len() - 1 {
            neighbours.push(Point {
                x: self.point.x + 1,
                y: self.point.y,
            });
        }

        if self.point.y < grid.len() - 1 {
            neighbours.push(Point {
                x: self.point.x,
                y: self.point.y + 1,
            });
        }

        if self.point.y > 0 {
            neighbours.push(Point {
                x: self.point.x,
                y: self.point.y - 1,
            });
        }

        if self.point.x > 0 {
            neighbours.push(Point {
                x: self.point.x - 1,
                y: self.point.y,
            });
        }

        neighbours
            .iter()
            .map(|&point| Position {
                letter: grid[point.y][point.x],
                point,
            })
            .filter(|point| is_valid(self.letter, point.letter))
            .collect()
    }
}

fn bfs(grid: &[Vec<char>], start: Position, end: Position) -> Option<usize> {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(start);
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((current, count)) = queue.pop_front() {
        if current == end {
            return Some(count);
        }
        for next in current.neighbours(grid) {
            if visited.insert(next) {
                queue.push_back((next, count + 1));
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

//
fn puzzle_1(input: &str) -> usize {
    let grid = parse_input(input);
    let start = Position::start(&grid);
    let end = Position::end(&grid);
    bfs(&grid, start, end).unwrap()
}

// Can we find a better starting point?
// We want to start as low as possible (a), yet be the shortest route to E.
fn puzzle_2(input: &str) -> usize {
    let grid = parse_input(input);
    let end = Position::end(&grid);
    let all_a = Position::letter_position(&grid, 'a');
    all_a
        .iter()
        .filter_map(|start| bfs(&grid, *start, end))
        .min()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn day_12_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
                vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
                vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i']
            ]
        );
    }

    #[test]
    fn day_12_start() {
        assert_eq!(
            Position::start(&parse_input(TEST_INPUT)),
            Position {
                point: Point { x: 0, y: 0 },
                letter: 'S'
            }
        );
    }

    #[test]
    fn day_12_end() {
        assert_eq!(
            Position::end(&parse_input(TEST_INPUT)),
            Position {
                point: Point { x: 5, y: 2 },
                letter: 'E'
            }
        )
    }

    #[test]
    fn day_12_neighbors() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(
            Position::start(&grid).neighbours(&grid),
            vec![
                Position {
                    point: Point { x: 1, y: 0 },
                    letter: 'a'
                },
                Position {
                    point: Point { x: 0, y: 1 },
                    letter: 'a'
                }
            ]
        );
        assert_eq!(
            Position {
                point: Point { x: 0, y: 1 },
                letter: 'a'
            }
            .neighbours(&grid),
            vec![
                Position {
                    point: Point { x: 1, y: 1 },
                    letter: 'b'
                },
                Position {
                    point: Point { x: 0, y: 2 },
                    letter: 'a'
                },
                Position {
                    point: Point { x: 0, y: 0 },
                    letter: 'S'
                }
            ]
        );
        assert_eq!(
            Position {
                point: Point { x: 1, y: 1 },
                letter: 'b'
            }
            .neighbours(&grid),
            vec![
                Position {
                    point: Point { x: 2, y: 1 },
                    letter: 'c'
                },
                Position {
                    point: Point { x: 1, y: 2 },
                    letter: 'c'
                },
                Position {
                    point: Point { x: 1, y: 0 },
                    letter: 'a'
                },
                Position {
                    point: Point { x: 0, y: 1 },
                    letter: 'a'
                }
            ]
        );
    }

    #[test]
    fn day_12_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 31);
    }

    #[test]
    fn day_12_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 29);
    }
}
