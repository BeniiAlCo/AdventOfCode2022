pub fn run() {
    let input = include_str!("input/day_8.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// We come across a patch of trees planted in a grid, as part of a reforestation effort.
// Would this location be good for making a tree house?
// To know this, we need to determine whether the tree house would be sufficiently hidden.
// To do this, we can count the number of trees that are visible from outside the grid when looking directly along a row or column.
//
// We have a list of heights of trees.
// Each tree is a single digit, ranging from 0-9
// A tree is VISIBLE if all other trees between it and an edge of the grid are shorter than it.
// All trees on the edge of the grid are visible.

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let size = rows.len();
    let cols = (0..size)
        .map(|i| rows.iter().map(|col| col[i]).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    (rows, cols)
}

// How many trees are visible?
fn is_visible_in_line(x: usize, row: &[usize]) -> bool {
    if row.iter().take(x).any(|r| r >= &row[x])
        && row
            .iter()
            .rev()
            .take(row.len() - x - 1)
            .any(|r| r >= &row[x])
    {
        return false;
    }

    true
}

fn is_visible((x, y): (usize, usize), row: Vec<usize>, col: Vec<usize>) -> bool {
    if x == 0 || y == 0 || x == row.len() - 1 || y == col.len() - 1 {
        true
    } else {
        is_visible_in_line(y, &row) || is_visible_in_line(x, &col)
    }
}

fn puzzle_1(input: &str) -> usize {
    let (rows, cols) = parse_input(input);
    let size = rows.len();

    (0..size)
        .map(|x| {
            (0..size)
                .map(|y| {
                    is_visible(
                        (x, y),
                        rows.to_owned()[x].to_owned(),
                        cols.to_owned()[y].to_owned(),
                    )
                })
                .filter(|&visible| visible)
                .count()
        })
        .sum()
}

// Now we need to find the best avaliable spot to place the tree house.
// The best spot has a view with lots of trees.
//
// A tree has a viewing distance from its position to an edge or to a tree atleast as tall.
//
// A tree's scenic score is each of its viewing distances multiplied together
//
// What is the highest scenic score of all trees?
fn distance(idx: usize, direction: &[usize]) -> usize {
    direction
        .iter()
        .skip(idx + 1)
        .position(|r| r >= &direction[idx])
        .map(|r| r + 1)
        .unwrap_or_else(|| direction[idx + 1..].len())
        * direction
            .iter()
            .rev()
            .skip(direction.len() - idx)
            .position(|r| r >= &direction[idx])
            .map(|r| r + 1)
            .unwrap_or_else(|| direction[..idx].len())
}

fn scenic_score((x, y): (usize, usize), row: Vec<usize>, col: Vec<usize>) -> usize {
    if x == 0 || y == 0 || x == row.len() - 1 || y == col.len() - 1 {
        0
    } else {
        distance(x, &col) * distance(y, &row)
    }
}

fn puzzle_2(input: &str) -> usize {
    let (rows, cols) = parse_input(input);
    let size = rows.len();

    *(0..size)
        .map(|x| {
            (0..size)
                .map(|y| {
                    scenic_score(
                        (x, y),
                        rows.to_owned()[x].to_owned(),
                        cols.to_owned()[y].to_owned(),
                    )
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
        .iter()
        .flatten()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn day_8_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            (
                vec![
                    vec![3, 0, 3, 7, 3],
                    vec![2, 5, 5, 1, 2],
                    vec![6, 5, 3, 3, 2],
                    vec![3, 3, 5, 4, 9],
                    vec![3, 5, 3, 9, 0]
                ],
                vec![
                    vec![3, 2, 6, 3, 3],
                    vec![0, 5, 5, 3, 5],
                    vec![3, 5, 3, 5, 3],
                    vec![7, 1, 3, 4, 9],
                    vec![3, 2, 2, 9, 0]
                ]
            )
        );
    }

    #[test]
    fn day_8_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 21);
    }

    #[test]
    fn day_8_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 8);
    }
}
