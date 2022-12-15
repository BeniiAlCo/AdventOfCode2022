use std::fmt;

pub fn run() {
    let input = include_str!("input/day_14.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// We've been led to a giant waterfall.
// A distress signal appears to be eminating from it.
// A path leads behind it, to a large cave system.
// As we enter the cave, there's a rockslide.
// We're going to analyze the path of falling material.
//
// We have a 2d vertical slice of cave above us.
// It is mostly air and rock structures.
// Our scan gives us the (x,y)
// - x : distance to the right
// - y : distance to the ground
//
// 0,0 1,0 2,0
// 1,0 1,1 2,1
// 2,0 1,2 2,2
//
// Our input is a series of (x,y)_0 -> (x,y)_1 -> ... (x,y)_n lists
// each list represents path formed by a solid rock structure
// The first point is a position, and all following are positions that form straight lines from the
// previous (vertical or horizontal)
//
// e.g. given:
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
//
// Two paths of rock -- one with two straight lines; one with three.
// Below, air is '.', rock is '#', sand entrance-point (read below) is '+':
//
//   4 4 4 5 5
//   9 9 9 0 0
//   4 6 8 0 2
// 0 ......+...
// 1 ..........
// 2 ..........
// 3 ..........
// 4 ....#...##
// 5 ....#...#.
// 6 ..###...#.
// 7 ........#.
// 8 ........#.
// 9 #########.

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn is_horizontal(&self, other: &Point) -> bool {
        self.y == other.y
    }

    fn is_vertical(&self, other: &Point) -> bool {
        self.x == other.x
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Element {
    Air,
    Rock,
    Sand,
    SandGenerator,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Air => '.',
                Element::Rock => '#',
                Element::Sand => 'o',
                Element::SandGenerator => '+',
            }
        )
    }
}

impl Default for Element {
    fn default() -> Self {
        Element::Air
    }
}

#[derive(Debug, PartialEq)]
struct Cave {
    array: Vec<Element>,
    width: usize,
    height: usize,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            let row = self.get_row(row).expect("This row did not exist: {row}");
            let mut line = String::new();
            for e in row {
                line.push_str(&format!("{e}"));
            }
            line.push('\n');
            write!(f, "{line}")?;
        }
        Ok(())
    }
}

impl Cave {
    fn new(width: usize, height: usize) -> Self {
        Self {
            array: [Element::default()].repeat(width * height),
            width,
            height,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, p: &Point) -> Option<&Element> {
        self.array.get(p.y * self.width + p.x)
    }

    fn get_row(&self, row: usize) -> Option<&[Element]> {
        let array = &self.array;
        let width = &self.width;
        array
            .get(row * width)
            .and(Some(&array[row * width..row * width + width]))
    }

    fn set(&mut self, p: Point, val: Element) {
        self.array[p.y * self.width + p.x] = val;
    }

    fn set_line(&mut self, start: Point, end: Point, val: Element) {
        if start.is_horizontal(&end) {
            let y = start.y;
            let (start, end) = if start.x <= end.x {
                (start, end)
            } else {
                (end, start)
            };
            for x in start.x..=end.x {
                self.set((x, y).into(), val);
            }
        } else {
            assert!(start.is_vertical(&end));
            let x = start.x;
            let (start, end) = if start.y <= end.y {
                (start, end)
            } else {
                (end, start)
            };
            for y in (start.y)..=(end.y) {
                self.set((x, y).into(), val);
            }
        }
    }

    fn find(&self, item: Element) -> Option<Point> {
        let position = self.array.iter().position(|&element| element == item);
        position.map(|pos| Point {
            x: pos % self.width,
            y: pos / self.width,
        })
    }

    fn check_below(&self, current: &Point) -> Option<Option<Point>> {
        let (down, left, right) = if current.x == 0 {
            (
                (current.x, current.y + 1).into(),
                (current.x, current.y + 1).into(),
                (current.x + 1, current.y + 1).into(),
            )
        } else if current.x == (self.width() - 1) {
            (
                (current.x, current.y + 1).into(),
                (current.x - 1, current.y + 1).into(),
                (current.x, current.y + 1).into(),
            )
        } else {
            (
                (current.x, current.y + 1).into(),
                (current.x - 1, current.y + 1).into(),
                (current.x + 1, current.y + 1).into(),
            )
        };
        let search = [down, left, right];

        if let Some(point) = search
            .iter()
            .find(|point| self.get(point) == Some(&Element::Air))
        {
            Some(Some(*point))
        } else if search.iter().all(|point| {
            self.get(point) == Some(&Element::Rock) || self.get(point) == Some(&Element::Sand)
        }) {
            if self.get(current) == Some(&Element::SandGenerator) {
                None
            } else {
                Some(None)
            }
        } else {
            None
        }
    }

    fn simulate_sand(&mut self, generator_position: Point) -> Option<Point> {
        let mut current = generator_position;

        while let Some(valid_space) = self.check_below(&current) {
            if let Some(open_space) = valid_space {
                current = open_space;
            } else {
                self.set(current, Element::Sand);
                return Some(current);
            }
        }

        None
    }
}

fn parse_input(input: &str) -> Cave {
    let input = input
        .lines()
        .map(|structure| {
            structure
                .split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()).into()
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let largest_y = input
        .iter()
        .flatten()
        .max_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y
        + 2;

    let width = (largest_y * 2) - 1;
    let height = largest_y + 1;
    let shift = 500 - ((width - 1) / 2);

    let input = input
        .iter()
        .map(|structure| {
            structure
                .iter()
                .map(|point| (point.x - shift, point.y).into())
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut cave = Cave::new(width, height);

    input.iter().for_each(|line| {
        line.windows(2).for_each(|pair| {
            cave.set_line(pair[0], pair[1], Element::Rock);
        });
    });
    cave.set(((500 - shift), 0).into(), Element::SandGenerator);

    cave
}

// Sand will fall into the cave at the point (500, 0) (marked with a '+' above).
// Sand is produced one unit at a time.
// Once one piece of sand is produced, another will not be until the previous has stopped moving.
// A piece of sand fills one co-ordinate point.
//
// The rules for sand movement are as follows:
// - attempt to move one step down.
// - If the space immediately below is occupied (by rock or sand), attempt to move diagonally.
// - - one step down, one step to the left.
// - - If the left is blocked, then one step down, one step right.
// - If directly down, diagonally left, and diagonally right are occupied, then finish moving.
//
// Simulate the falling sand.
// How many pieces of sand must settle on the rock structures before all future sand would flow
// below them?
fn puzzle_1(input: &str) -> usize {
    let mut cave = parse_input(input);
    let mut counter = 0;

    let sand_generator = cave.find(Element::SandGenerator).unwrap();

    while let Some(_resting_point) = cave.simulate_sand(sand_generator) {
        counter += 1;
    }

    counter
}

// Now there's a floor!
fn puzzle_2(input: &str) -> u64 {
    let mut cave = parse_input(input);
    cave.set_line(
        (0, cave.height() - 1).into(),
        (cave.width() - 1, cave.height() - 1).into(),
        Element::Rock,
    );

    let mut counter = 0;

    let sand_generator = cave.find(Element::SandGenerator).unwrap();

    while let Some(_resting_point) = cave.simulate_sand(sand_generator) {
        counter += 1;
    }

    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn day_14_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 24);
    }

    #[test]
    fn day_14_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 93);
    }
}
