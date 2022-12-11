pub fn run() {
    let input = include_str!("input/day_11.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// Monkeys are playing with our belongings.
// We want to predict where the monkeys will throw them.
//
// Each monkey operates differently based on how worried we are about each item.
// Our input is a list of monkeys, each containing a list of items they currently have; an
// operation; and a test.
//
// - Starting items : lists worry level for each item (not sorted)
// - Operation : how worry level will change in the form, "new = <operation>"
// - Test : how the monkey will use the worry level, in the form, "(if) <test>\nIf true: throw to
// monkey <x>\nIf false: throw the monkey <y>".
//
// After each monkey inspects an item, but before it tests, divide current worry level by 3,
// rounded down.
//
// Monkeys take turns inspecting and throwing items.
// On a monkeys turn, it inspects and throws each item, one at a time, in the order stated.
// A round is made up of each monkeys turn.
//
// When a monkey throws an item, it goes on the end of the receiving monkey's list.

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    test_no: usize,
    inspection_count: usize,
}
impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.items,)
    }
}
fn parse_input(input: &str) -> Vec<Monkey> {
    fn parse_operation((sign, operand): (&str, &str)) -> Box<dyn Fn(usize) -> usize> {
        if let Ok(operand) = operand.parse::<usize>() {
            match sign {
                "+" => Box::new(move |old| old + operand),
                "*" => Box::new(move |old| old * operand),
                _ => unreachable!(),
            }
        } else {
            match sign {
                "+" => Box::new(|old| old + old),
                "*" => Box::new(|old| old * old),
                _ => unreachable!(),
            }
        }
    }

    fn parse_test(test: usize, t: usize, f: usize) -> Box<dyn Fn(usize) -> usize> {
        Box::new(move |worry| if worry % test == 0 { t } else { f })
    }

    input
        .split("\n\n")
        .map(|monkey| {
            let mut monkey = monkey.split('\n').skip(1);
            let items = monkey
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let operation = parse_operation(
                monkey
                    .next()
                    .unwrap()
                    .trim_start_matches("  Operation: new = old ")
                    .split_once(' ')
                    .unwrap(),
            );
            let test = monkey
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse::<usize>()
                .unwrap();
            let test_no = test;
            let t = monkey
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse::<usize>()
                .unwrap();
            let f = monkey
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse::<usize>()
                .unwrap();
            let test = parse_test(test, t, f);

            Monkey {
                items,
                operation,
                test,
                test_no,
                inspection_count: 0,
            }
        })
        .collect::<Vec<Monkey>>()
}

//
fn puzzle_1(input: &str) -> usize {
    fn relief(item: usize) -> usize {
        item / 3
    }
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            for idx in 0..monkeys[monkey].items.len() {
                let mut item = monkeys[monkey].items[idx];
                item = (monkeys[monkey].operation)(item);
                item = relief(item);
                let new_monkey = (monkeys[monkey].test)(item);
                monkeys[new_monkey].items.push(item);
            }
            monkeys[monkey].inspection_count += monkeys[monkey].items.len();
            monkeys[monkey].items.clear();
        }
    }
    let mut inspection_count = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();
    inspection_count.sort();
    inspection_count[inspection_count.len() - 1] * inspection_count[inspection_count.len() - 2]
}

//
fn puzzle_2(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    let modulo: usize = monkeys.iter().map(|m| m.test_no).product();
    for _ in 0..10_000 {
        for monkey in 0..monkeys.len() {
            for idx in 0..monkeys[monkey].items.len() {
                let mut item = monkeys[monkey].items[idx];
                item = (monkeys[monkey].operation)(item);
                item %= modulo;
                let new_monkey = (monkeys[monkey].test)(item);
                monkeys[new_monkey].items.push(item);
            }
            monkeys[monkey].inspection_count += monkeys[monkey].items.len();
            monkeys[monkey].items.clear();
        }
    }
    let mut inspection_count = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();
    inspection_count.sort();
    inspection_count[inspection_count.len() - 1] * inspection_count[inspection_count.len() - 2]
}
