fn main() {
    if let Err(err) = aoc_2022::Config::new().and_then(aoc_2022::Config::run) {
        eprintln!("{}", err);
        std::process::exit(1)
    }
    std::process::exit(0)
}
