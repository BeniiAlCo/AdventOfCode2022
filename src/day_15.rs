pub fn run() {
    let input = include_str!("input/day_15.txt");
    println!("{}", puzzle_1(input, 2_000_000));
    println!("{}", puzzle_2(input, 4_000_000));
}

// We've been led to a network of subterranean tunnels.
// We need to search them, but it will take too long to seach each individually.
// We can use a deployable sensor used to locate lost elves.
// The sensor will monitor for signs of a beacon we are trying to find.
//
// A sensor knows its own position, and the position of the nearest beacon (never ties).
// The nearest beacon is determined via Manhattan Distance.
//
// We are given a series of sensor position reports of the form:
// 'Sensor at x=<s_x>, y=<s_y>: closest beacon is at x=<b_x>, y=<b_y>'
//
// We can map out all sensors and their corresponding closest beacons using this data.
// This does not mean we can use it to map out ALL beacons -- some could exist in positions not
// picked up by sensors.
//
// None of the detected beacons is the one that we are searching for.

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance_to(&self, rhs: &Point) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    fn parse_point(input: &str) -> Self {
        // expected form: 'x=<x>, y=<y>'
        let (x, y) = input
            .strip_prefix("x=")
            .unwrap()
            .split_once(", y=")
            .unwrap();
        Point {
            x: x.parse::<i64>().unwrap(),
            y: y.parse::<i64>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    nearest_beacon: Point,
    distance_to_beacon: i64,
}

impl Sensor {
    fn new(location: Point, nearest_beacon: Point, distance_to_beacon: i64) -> Self {
        Sensor {
            location,
            nearest_beacon,
            distance_to_beacon,
        }
    }

    fn can_contain_unseen_points(&self, min: Point, max: Point) -> bool {
        let corners = [
            (min.x, min.y),
            (min.x, max.y),
            (max.x, min.y),
            (max.x, max.y),
        ];
        let largetst_dist = corners
            .iter()
            .map(|corner| (corner.0 - self.location.x).abs() + (corner.1 - self.location.y).abs())
            .max()
            .unwrap();
        largetst_dist > self.distance_to_beacon
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    fn parse_pairs((sensor, beacon): (&str, &str)) -> (Point, Point) {
        (Point::parse_point(sensor), Point::parse_point(beacon))
    }

    input
        .lines()
        .map(|line| {
            let (sensor, beacon) = parse_pairs(
                line.strip_prefix("Sensor at ")
                    .unwrap()
                    .split_once(": closest beacon is at ")
                    .unwrap(),
            );
            Sensor::new(sensor, beacon, sensor.manhattan_distance_to(&beacon))
        })
        .collect::<Vec<Sensor>>()
}

// Count the positions where a beacon cannot possibly be, along a single row.
// How many positions cannot hold a beacon in the row where y=2_000_000
fn puzzle_1(input: &str, target_row: i64) -> usize {
    use std::collections::HashSet;

    let sensors = parse_input(input);
    let mut on_target = HashSet::new();

    sensors.iter().for_each(|sensor| {
        if sensor.location.manhattan_distance_to(&Point {
            x: sensor.location.x,
            y: target_row,
        }) <= sensor.distance_to_beacon
        {
            let distance_to_target = (sensor.location.y - target_row).abs();
            let range = sensor.distance_to_beacon - distance_to_target;

            if range >= 0 {
                (-range..=range).for_each(|dx| {
                    on_target.insert(Point {
                        x: sensor.location.x + dx,
                        y: target_row,
                    });
                });
            }
        }
    });

    sensors.iter().for_each(|sensor| {
        if sensor.nearest_beacon.y == target_row {
            on_target.remove(&sensor.nearest_beacon);
        }
    });

    on_target.len()
}

// Now we're finding the one point in a region that is unexplored by the surrounding sensors.
// Use that position to find the answer!
fn puzzle_2(input: &str, target_region_size: i64) -> i64 {
    let min = Point { x: 0, y: 0 };
    let max = Point {
        x: target_region_size,
        y: target_region_size,
    };

    let sensors = parse_input(input);
    let mut quadrant_stack = vec![(min, max)];
    let mut position = None;

    while let Some((min, max)) = quadrant_stack.pop() {
        if min == max {
            if sensors.iter().all(|sensor| {
                sensor.location.manhattan_distance_to(&min) > sensor.distance_to_beacon
            }) {
                position = Some(min);
            }
        } else {
            let mid = Point {
                x: (min.x + max.x) / 2,
                y: (min.y + max.y) / 2,
            };
            let quadrants = [
                (min, mid),
                (
                    Point {
                        x: mid.x + 1,
                        y: min.y,
                    },
                    Point { x: max.x, y: mid.y },
                ),
                (
                    Point {
                        x: min.x,
                        y: mid.y + 1,
                    },
                    Point { x: mid.x, y: max.y },
                ),
                (
                    Point {
                        x: mid.x + 1,
                        y: mid.y + 1,
                    },
                    max,
                ),
            ];

            for quad in quadrants.iter() {
                if quad.0.x > quad.1.x || quad.0.y > quad.1.y {
                    continue;
                }
                if sensors
                    .iter()
                    .all(|pair| pair.can_contain_unseen_points(quad.0, quad.1))
                {
                    quadrant_stack.push(*quad);
                }
            }
        }
    }

    if let Some(position) = position {
        (position.x * target_region_size) + position.y
    } else {
        panic!()
    }
}
