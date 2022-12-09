use std::collections::HashSet;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

pub fn run() {
    let input = include_str!("input/day_9.txt");
    println!("{}", puzzle_1(input));
    //println!("{}", puzzle_2(input));
}

// There's a rope bridge.
// We want to figure out where not to walk.
//
// Consider a rope.
// There's a knot at each end.
// Call one H(ead), and the other T(ail).
// If H moves far enough away from T, H drags T towards it.
// H and T must always be touching (diagonally or overlapping count).
// if H is ever two steps from T, T moves once in that direction.
// Assume H and T start in the same position.
//
// Given a 2D grid and a series of movements for H, determine the position of T.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn update(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn within_one(&self, other: &Self) -> bool {
        self.x <= other.x + 1
            && self.x >= other.x - 1
            && self.y <= other.y + 1
            && self.y >= other.y - 1
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, other: i32) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

struct Rope {
    head: Point,
    tail: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Motion {
    direction: Direction,
    distance: usize,
}

impl Rope {
    fn apply_move(&mut self, motion: &Motion) {
        //(0..motion.distance).for_each(|_| {
        self.head.update(motion.direction);
        if let Some(new_tail) = self.check_tail(motion) {
            self.tail = new_tail;
        }
        //  println!("head:{:?}, tail:{:?}", self.head, self.tail);
        //})
    }

    fn check_tail(&self, motion: &Motion) -> Option<Point> {
        
        if self.head == self.tail || self.head.within_one(&self.tail) {
            None
        } else {
            let mut tail = self.tail;
            if self.head.x == self.tail.x || self.head.y == self.tail.y {
                tail.update(motion.direction);
                Some(tail)
            } else {
                if (self.head.x - self.tail.x).abs() == 2 {
                    tail.update(motion.direction);
                    if tail.y + 1 == self.head.y {
                        Some(Point {
                            x: tail.x,
                            y: tail.y + 1,
                        })
                    } else {
                        Some(Point {
                            x: tail.x,
                            y: tail.y - 1,
                        })
                    }
                } else {
                    tail.update(motion.direction);
                    if tail.x + 1 == self.head.x {
                        Some(Point {
                            x: tail.x + 1,
                            y: tail.y,
                        })
                    } else {
                        Some(Point {
                            x: tail.x - 1,
                            y: tail.y,
                        })
                    }
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            let direction = match direction {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!(),
            };
            let distance = distance.parse::<usize>().unwrap();
            Motion {
                direction,
                distance,
            }
        })
        .collect::<Vec<Motion>>()
}

//
fn puzzle_1(input: &str) -> usize {
    let mut rope = Rope {
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
    };
    let mut set = HashSet::new();
    parse_input(input).iter().for_each(|m| {
        (0..m.distance).for_each(|_| {
            set.insert(rope.tail);
            rope.apply_move(m);
        })
    });
    set.insert(rope.tail);
    set.len()
}

// The rope snaps.
// Now consider ropes with 10 knots, rather than 2.
// One is the head, and has a tail. That tail is also a head, and so on.
fn puzzle_2(input: &str) -> u64 {
    let mut ropes = vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn day_9_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                Motion {
                    direction: Direction::Right,
                    distance: 4
                },
                Motion {
                    direction: Direction::Up,
                    distance: 4
                },
                Motion {
                    direction: Direction::Left,
                    distance: 3
                },
                Motion {
                    direction: Direction::Down,
                    distance: 1
                },
                Motion {
                    direction: Direction::Right,
                    distance: 4
                },
                Motion {
                    direction: Direction::Down,
                    distance: 1
                },
                Motion {
                    direction: Direction::Left,
                    distance: 5
                },
                Motion {
                    direction: Direction::Right,
                    distance: 2
                }
            ]
        );
    }

    #[test]
    fn day_9_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 11);
    }

    #[test]
    fn day_9_puzzle_2() {
        //	assert_eq!(puzzle_2(TEST_INPUT), 0);
    }
}
// 00 00
// 10 00
// 20 10
// 30 20
// 40 30
// 41 30
// 42 41
// 43 42
// 44 43
// 34 43
// 24 34
// 14 24
// 13 24
// 23 24
// 33 24
// 43 33
// 53 43
// 52 43
//
