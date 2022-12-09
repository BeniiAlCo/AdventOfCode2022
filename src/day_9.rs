use std::collections::HashSet;

pub fn run() {
    let input = include_str!("input/day_9.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> Vec<(&str, usize)> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            (direction, distance.parse::<usize>().unwrap())
        })
        .collect::<Vec<(&str, usize)>>()
}

fn puzzle_1(input: &str) -> usize {
    let mut rope = vec![Point { x: 0, y: 0 }; 2];

    let mut visited_set = HashSet::new();
    visited_set.insert(rope[1]);

    for (direction, distance) in parse_input(input) {
        for _ in 0..distance {
            let mut head = rope[0];
            let mut tail = rope[1];

            match direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => unreachable!(),
            };

            if (head.y - tail.y).abs() > 1 || (head.x - tail.x).abs() > 1 {
                tail.x += (head.x - tail.x).signum();
                tail.y += (head.y - tail.y).signum();
            }

            rope[0] = head;
            rope[1] = tail;

            visited_set.insert(rope[1]);
        }
    }
    visited_set.len()
}

fn puzzle_2(input: &str) -> usize {
    let mut rope = vec![Point { x: 0, y: 0 }; 10];

    let mut visited_set = HashSet::new();
    visited_set.insert(rope[9]);

    for (direction, distance) in parse_input(input) {
        for _ in 0..distance {
            for idx in 0..(rope.len() - 1) {
                let mut head = rope[idx];
                let mut tail = rope[idx + 1];

                if idx == 0 {
                    match direction {
                        "R" => head.x += 1,
                        "L" => head.x -= 1,
                        "U" => head.y += 1,
                        "D" => head.y -= 1,
                        _ => unreachable!(),
                    }
                };

                if (head.y - tail.y).abs() > 1 || (head.x - tail.x).abs() > 1 {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                }

                rope[idx] = head;
                rope[idx + 1] = tail;
            }

            visited_set.insert(rope[9]);
        }
    }
    visited_set.len()
}
