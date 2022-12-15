macro_rules! run {
    ( $day:expr ) => {
        match $day {
            1 => day_1::run(),
            2 => day_2::run(),
            3 => day_3::run(),
            4 => day_4::run(),
            5 => day_5::run(),
            6 => day_6::run(),
            7 => day_7::run(),
            8 => day_8::run(),
            9 => day_9::run(),
            10 => day_10::run(),
            11 => day_11::run(),
            12 => day_12::run(),
            13 => day_13::run(),
            14 => day_14::run(),
            _ => unreachable!(),
        }
    };
}

use clap::{Arg, ArgAction, Command};
use std::ops::RangeInclusive;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

pub struct Config {
    all_days: bool,
    specific_day: usize,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let matches = Command::new("AoC_2022")
            .author("BeniiAlCo")
            .about("Advent of Code 2022 in Rust!")
            .arg(
                Arg::new("AllDays")
                    .long("all_days")
                    .short('a')
                    .num_args(0)
                    .exclusive(true)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("SpecificDay")
                    .long("day")
                    .short('d')
                    .num_args(1)
                    .exclusive(true)
                    .value_parser(Self::days_in_range),
            )
            .get_matches();

        Ok(Config {
            all_days: matches.get_flag("AllDays"),
            specific_day: if matches.contains_id("SpecificDay") {
                *matches.get_one::<usize>("SpecificDay").unwrap()
            } else {
                *Self::DAY_RANGE.end()
            },
        })
    }

    const DAY_RANGE: RangeInclusive<usize> = 1..=14;

    fn days_in_range(s: &str) -> Result<usize, String> {
        let days: usize = s
            .parse()
            .map_err(|_| format!("'{}' isn't a valid Advent of Code day for 2022!", s))?;
        if Self::DAY_RANGE.contains(&days) {
            Ok(days)
        } else {
            Err(format!(
                "Day not in range {}-{}",
                Self::DAY_RANGE.start(),
                Self::DAY_RANGE.end()
            ))
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.all_days {
            Self::DAY_RANGE.for_each(|day| run!(day));
        } else {
            run!(self.specific_day);
        }
        Ok(())
    }
}
