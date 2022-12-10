pub fn run() {
    let input = include_str!("input/day_10.txt");
    println!("{}", puzzle_1(input));
    println!("{}", puzzle_2(input));
}

// We want to design a replacement for a broken video display.
// The display is driven by a clock circuit, ticking at a constant rate.
// Call a single tick a cycle.
//
// - What signal is being sent by the CPU?
// The CPU has a single register.
// Call it x.
// x starts with the value 1.
// The CPU has two instructions.
// - addx v : takes two cycles; x is increased by value v.
// - noop : takes one cycle; no effect.
//
// We will receive a program using these instructions.
// Use the output to determine a video output.
//
// Determine the video output by considering the signal strength during the 20th cycle, and every
// 40 cycles after that.
// Signal strength is cycle number * x

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.trim() == "noop" {
                Instruction::Noop
            } else {
                Instruction::AddX(line.trim_start_matches("addx ").parse::<i64>().unwrap())
            }
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i64),
}

struct Cpu {
    cycle: usize,
    x: i64,
    current_instruction: Option<Instruction>,
    cycles_until_instruction: usize,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cycle: 0,
            x: 1,
            current_instruction: None,
            cycles_until_instruction: 0,
        }
    }

    fn apply_instruction(&mut self) {
        if let Some(instruction) = &self.current_instruction {
            match instruction {
                Instruction::Noop => {}
                Instruction::AddX(v) => {
                    self.x += v;
                }
            }
        }
        self.current_instruction = None;
        self.cycles_until_instruction = 0;
    }

    fn next_cycle(&mut self) {
        self.cycles_until_instruction -= 1;
        self.cycle += 1;

        if self.cycles_until_instruction == 0 {
            self.apply_instruction();
        }
    }

    fn run_1(&mut self, instructions: Vec<Instruction>) -> usize {
        let mut signal = 0;
        let instructions = instructions.iter();
        for instruction in instructions {
            if self.current_instruction.is_none() {
                self.current_instruction = Some(*instruction);
            }
            self.cycles_until_instruction = if let Instruction::Noop = instruction {
                1
            } else {
                2
            };

            while self.cycles_until_instruction != 0 {
                if (self.cycle + 1) == 20
                    || (self.cycle + 1) > 20 && ((self.cycle + 1) - 20) % 40 == 0
                {
                    signal += self.x * (self.cycle + 1) as i64;
                    //println!("{}:{},{}", self.cycle, self.x, signal);
                }
                self.next_cycle();

                //if self.cycle > 180 {
                //    println!("{}:{}", self.cycle, self.x);
                //}
            }
        }
        signal as usize
    }

    fn run_2(&mut self, instructions: Vec<Instruction>) -> String {
        let mut display = String::new();
        let instructions = instructions.iter();
        for instruction in instructions {
            if self.current_instruction.is_none() {
                self.current_instruction = Some(*instruction);
            }
            self.cycles_until_instruction = if let Instruction::Noop = instruction {
                1
            } else {
                2
            };

            while self.cycles_until_instruction != 0 {
                self.cycles_until_instruction -= 1;
                self.cycle += 1;

                let sprite = [self.x - 1, self.x, self.x + 1];

                if sprite.iter().any(|&s| s == (self.cycle - 1) as i64 % 40) {
                    display.push('#')
                } else {
                    display.push('.')
                }

                //dbg!(&display);
                if self.cycles_until_instruction == 0 {
                    self.apply_instruction();
                }
            }
        }
        (0..6)
            .map(|n| {
                let mut line = display.chars().skip(n * 40).take(40).collect::<String>();
                line.push('\n');
                line
            })
            .collect::<String>()
    }
}

// Calulate the sum of (cycle * x), where cycle = 20, 60, 100, 140, 180, 220
fn puzzle_1(input: &str) -> usize {
    let mut cpu = Cpu::new();
    let instructions = parse_input(input);
    cpu.run_1(instructions)
}

// x controls the horizontal position of a sprite.
// The sprite is 3 pixels wide.
// X sets the horizontal position of the middle sprite.
// the output is 40 pixels wide & 6 pixels tall.
// It draws left to right, top to bottom.
// left-most position is 0; right-most is 39.
//
// A single pixel is drawn each cycle:
// Cycle   1 -> ######################################## <- Cycle  40
// Cycle  41 -> ######################################## <- Cycle  80
// Cycle  81 -> ######################################## <- Cycle 120
// Cycle 121 -> ######################################## <- Cycle 160
// Cycle 161 -> ######################################## <- Cycle 200
// Cycle 201 -> ######################################## <- Cycle 240
//
fn puzzle_2(input: &str) -> String {
    let mut cpu = Cpu::new();
    let instructions = parse_input(input);
    cpu.run_2(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "noop
addx 3
addx -5";
    const TEST_INPUT_1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn day_10_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                Instruction::Noop,
                Instruction::AddX(3),
                Instruction::AddX(-5)
            ]
        );
    }

    #[test]
    fn day_10_puzzle_1() {
        assert_eq!(puzzle_1(TEST_INPUT), 0);
    }

    #[test]
    fn day_10_puzzle_1_1() {
        assert_eq!(puzzle_1(TEST_INPUT_1), 13140)
    }

    #[test]
    fn day_10_puzzle_2() {
        assert_eq!(
            puzzle_2(TEST_INPUT_1),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n"
                .to_string()
        );
    }
}
