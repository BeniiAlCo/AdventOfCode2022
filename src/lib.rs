mod day1;

pub struct Config {
    all_days: bool,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config { all_days: true })
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.all_days {
            day1::run();
        }
        Ok(())
    }
}
