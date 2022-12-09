use std::collections::HashMap;

pub fn run() {
    let input = include_str!("input/day_7.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// We are trying to clear storage space in a malfunctioning device to be able to install an update.
// Our input will be a series of termial prompts and their subsequent outputs.
// Lines that begin with '$' are our prompted commands; lines without are their outputs.
//
// Possible commands are:
// - cd <location>: change directory
// - - cd x : move into directory x in the current directory
// - - cd .. : move into the directory that contains the current directory
// - - cd / : move into the outer-most directory
// - ls : list
// - - 123 abc : the current directory contains a file named 'abc', which has a size of '123'
// - - dir xyz : the current directory contains a directory named 'xyz'
//
// The file system of the device consists of a tree of plain data files and directories.
// The outer-most directory is called '/'.

fn parse_input(input: &str) -> HashMap<String, usize> {
    let mut current_path = String::new();
    let mut map = HashMap::new();

    current_path.push_str("~/");
    map.entry("~/".to_string()).or_insert(0);
    input.split('$').skip(2).for_each(|segment| {
        let (command, output) = segment.trim_start().split_once('\n').unwrap();
        if command.contains("cd") {
            if command.contains("..") {
                if let Some((current, _)) = current_path.rsplit_once('/') {
                    current_path = current.to_string();
                    if current_path.is_empty() {
                        current_path.push('/')
                    }
                }
            } else {
                if current_path != "~/" {
                    current_path.push('/')
                };

                current_path.push_str(command.strip_prefix("cd ").unwrap());
            }
        }
        output.lines().for_each(|output| {
            if output.starts_with("dir") {
                let mut dir_path = current_path.to_owned();
                if dir_path != "~/" {
                    dir_path.push('/')
                }
                dir_path.push_str(output.strip_prefix("dir ").unwrap());
                map.insert(dir_path, 0);
            } else {
                for (dir, val) in &mut map {
                    if current_path.starts_with(&**dir) {
                        *val += output
                            .split_whitespace()
                            .next()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap();
                    }
                }
            }
        });
    });
    map
}

// What is the sum of the total sizes of the directories whose total sizes are at most 100,000
fn puzzle_1(input: &str) -> usize {
    parse_input(input)
        .values()
        .filter(|&&val| val <= 100_000)
        .sum()
}

// Total disk space = 70_000_000
// Total needed = 30_000_000
// We need to find a directory that will free enough space.
fn puzzle_2(input: &str) -> usize {
    let map = parse_input(input);
    let req = 30_000_000 - (70_000_000 - map.get("~/").unwrap());
    *map.values().filter(|&&val| val >= req).min().unwrap()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const TEST_INPUT: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    #[test]
    fn day_7_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            HashMap::from([
                ("~/".to_string(), 48381165),
                ("~/a".to_string(), 94853),
                ("~/a/e".to_string(), 584),
                ("~/d".to_string(), 24933642)
            ])
        );
    }

    #[test]
    fn day_7_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 95437);
    }

    #[test]
    fn day_7_puzzle_2() {
        assert_eq!(puzzle_2(TEST_INPUT), 24933642);
    }
}
