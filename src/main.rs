use clap::Parser;
use aoc::{aoc_2015, aoc_2021};

#[derive(Parser)]
#[clap(version = "1.0")]
#[derive(Debug)]
struct Opts {
    year: u32,
    day: u32,
    part: u32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let input = std::io::stdin();
    // let input = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect();

    match (opts.year, opts.day, opts.part) {
        (2015, 1, 1) =>aoc_2015::day1_part1(input),
        (2015, 1, 2) =>aoc_2015::day1_part2(input),
        (2021, 1, 1) =>aoc_2021::day1_part1(input),
        (2021, 1, 2) =>aoc_2021::day1_part1(input),
        (2021, 2, 1) =>aoc_2021::day2_part1(input),
        (2021, 2, 2) =>aoc_2021::day2_part2(input),
        _ => eprintln!("Error: Unknown options {:?}", opts),
    }
}
